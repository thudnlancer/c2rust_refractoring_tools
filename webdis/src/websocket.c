#include "sha1/sha1.h"
#include <b64/cencode.h>
#include "acl.h"
#include "websocket.h"
#include "client.h"
#include "cmd.h"
#include "worker.h"
#include "pool.h"
#include "http.h"
#include "slog.h"
#include "server.h"
#include "conf.h"

/* message parsers */
#include "formats/json.h"
#include "formats/raw.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <sys/param.h>

static int
ws_schedule_write(struct ws_client *ws);

/**
 * This code uses the WebSocket specification from RFC 6455.
 * A copy is available at http://www.rfc-editor.org/rfc/rfc6455.txt
 */
#if __BIG_ENDIAN__
# define webdis_htonll(x) (x)
# define webdis_ntohll(x) (x)
#else
# define webdis_htonll(x) (((uint64_t)htonl((x) & 0xFFFFFFFF) << 32) | htonl((x) >> 32))
# define webdis_ntohll(x) (((uint64_t)ntohl((x) & 0xFFFFFFFF) << 32) | ntohl((x) >> 32))
#endif

static int
ws_compute_handshake(struct http_client *c, char *out, size_t *out_sz) {

	unsigned char *buffer, sha1_output[20];
	char magic[] = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
	SHA1Context ctx;
	base64_encodestate b64_ctx;
	int pos, i;

	// websocket handshake
	const char *key = client_get_header(c, "Sec-WebSocket-Key");
	size_t key_sz = key?strlen(key):0, buffer_sz = key_sz + sizeof(magic) - 1;
	if(!key || key_sz < 16 || key_sz > 32) { /* supposed to be exactly 16 bytes that were b64 encoded */
		slog(c->s, WEBDIS_WARNING, "Invalid Sec-WebSocket-Key", 0);
		return -1;
	}
	buffer = calloc(buffer_sz, 1);
	if(!buffer) {
		slog(c->s, WEBDIS_ERROR, "Failed to allocate memory for WS header", 0);
		return -1;
	}

	// concatenate key and guid in buffer
	memcpy(buffer, key, key_sz);
	memcpy(buffer+key_sz, magic, sizeof(magic)-1);

	// compute sha-1
	SHA1Reset(&ctx);
	SHA1Input(&ctx, buffer, buffer_sz);
	SHA1Result(&ctx);
	for(i = 0; i < (int)(20/sizeof(int)); ++i) {	// put in correct byte order before memcpy.
		ctx.Message_Digest[i] = ntohl(ctx.Message_Digest[i]);
	}
	memcpy(sha1_output, ctx.Message_Digest, 20);

	// encode `sha1_output' in base 64, into `out'.
	base64_init_encodestate(&b64_ctx);
	pos = base64_encode_block((const char*)sha1_output, 20, out, &b64_ctx);
	base64_encode_blockend(out + pos, &b64_ctx);

	// compute length, without \n
	*out_sz = strlen(out);
	if(out[*out_sz-1] == '\n')
		(*out_sz)--;

	free(buffer);

	return 0;
}

struct ws_client *
ws_client_new(struct http_client *http_client) {

	int db_num = http_client->w->s->cfg->database;
	struct ws_client *ws = calloc(1, sizeof(struct ws_client));
	struct evbuffer *rbuf = evbuffer_new();
	struct evbuffer *wbuf = evbuffer_new();
	redisAsyncContext *ac = pool_connect(http_client->w->pool, db_num, 0);

	if(!ws || !rbuf || !wbuf) {
		slog(http_client->s, WEBDIS_ERROR, "Failed to allocate memory for WS client", 0);
		if(ws) free(ws);
		if(rbuf) evbuffer_free(rbuf);
		if(wbuf) evbuffer_free(wbuf);
		if(ac) redisAsyncFree(ac);
		return NULL;
	}

	http_client->ws = ws;
	ws->http_client = http_client;
	ws->rbuf = rbuf;
	ws->wbuf = wbuf;
	ws->ac = ac;

	return ws;
}

static void
ws_client_free(struct ws_client *ws) {

	/* mark WS client as closing to skip the Redis callback */
	ws->close_after_events = 1;
	pool_free_context(ws->ac); /* could trigger a cb via format_send_error */

	struct http_client *c = ws->http_client;
	if(c) {
		close(c->fd);
		c->ws = NULL; /* detach if needed */
	}
	evbuffer_free(ws->rbuf);
	evbuffer_free(ws->wbuf);
	if(ws->cmd) {
		ws->cmd->ac = NULL; /* we've just free'd it */
		cmd_free(ws->cmd);
	}
	free(ws);
	if(c) http_client_free(c);
}


int
ws_handshake_reply(struct ws_client *ws) {

	struct http_client *c = ws->http_client;
	char sha1_handshake[40];
	char *buffer = NULL, *p;
	const char *origin = NULL, *host = NULL;
	size_t origin_sz = 0, host_sz = 0, handshake_sz = 0, sz;

	char template_start[] = "HTTP/1.1 101 Switching Protocols\r\n"
		"Upgrade: websocket\r\n"
		"Connection: Upgrade";
	char template_accept[] = "\r\n" /* just after the start */
		"Sec-WebSocket-Accept: "; /* %s */
	char template_sec_origin[] = "\r\n"
		"Sec-WebSocket-Origin: "; /* %s (optional header) */
	char template_loc[] = "\r\n"
		"Sec-WebSocket-Location: ws://"; /* %s%s */
	char template_end[] = "\r\n\r\n";

	if((origin = client_get_header(c, "Origin"))) {
		origin_sz = strlen(origin);
	} else if((origin = client_get_header(c, "Sec-WebSocket-Origin"))) {
		origin_sz = strlen(origin);
	}
	if((host = client_get_header(c, "Host"))) {
		host_sz = strlen(host);
	}

	/* need those headers */
	if(!host || !host_sz || !c->path || !c->path_sz) {
		slog(c->s, WEBDIS_WARNING, "Missing headers for WS handshake", 0);
		return -1;
	}

	memset(sha1_handshake, 0, sizeof(sha1_handshake));
	if(ws_compute_handshake(c, &sha1_handshake[0], &handshake_sz) != 0) {
		/* failed to compute handshake. */
		slog(c->s, WEBDIS_WARNING, "Failed to compute handshake", 0);
		return -1;
	}

	sz = sizeof(template_start)-1
		+ sizeof(template_accept)-1 + handshake_sz
		+ (origin && origin_sz ? (sizeof(template_sec_origin)-1 + origin_sz) : 0) /* optional origin */
		+ sizeof(template_loc)-1 + host_sz + c->path_sz
		+ sizeof(template_end)-1;

	p = buffer = malloc(sz);
	if(!p) {
		slog(c->s, WEBDIS_ERROR, "Failed to allocate buffer for WS handshake", 0);
		return -1;
	}

	/* Concat all */

	/* template_start */
	memcpy(p, template_start, sizeof(template_start)-1);
	p += sizeof(template_start)-1;

	/* template_accept */
	memcpy(p, template_accept, sizeof(template_accept)-1);
	p += sizeof(template_accept)-1;
	memcpy(p, &sha1_handshake[0], handshake_sz);
	p += handshake_sz;

	/* template_sec_origin */
	if(origin && origin_sz) {
		memcpy(p, template_sec_origin, sizeof(template_sec_origin)-1);
		p += sizeof(template_sec_origin)-1;
		memcpy(p, origin, origin_sz);
		p += origin_sz;
	}

	/* template_loc */
	memcpy(p, template_loc, sizeof(template_loc)-1);
	p += sizeof(template_loc)-1;
	memcpy(p, host, host_sz);
	p += host_sz;
	memcpy(p, c->path, c->path_sz);
	p += c->path_sz;

	/* template_end */
	memcpy(p, template_end, sizeof(template_end)-1);
	p += sizeof(template_end)-1;

	int add_ret = evbuffer_add(ws->wbuf, buffer, sz);
	free(buffer);
	if(add_ret < 0) {
		slog(c->s, WEBDIS_ERROR, "Failed to add response for WS handshake", 0);
		return -1;
	}

	return ws_schedule_write(ws); /* will free buffer and response once sent */
}

static void
ws_log_cmd(struct ws_client *ws, struct cmd *cmd) {
	char log_msg[SLOG_MSG_MAX_LEN];
	char *p = log_msg, *eom = log_msg + sizeof(log_msg) - 1;
	if(!slog_enabled(ws->http_client->s, WEBDIS_DEBUG)) {
		return;
	}

	memset(log_msg, 0, sizeof(log_msg));
	memcpy(p, "WS: ", 4); /* WS prefix */
	p += 4;

	for(int i = 0; p < eom && i < cmd->count; i++) {
		*p++ = '/';
		char *arg = cmd->argv[i];
		size_t arg_sz = cmd->argv_len[i];
		size_t copy_sz = arg_sz < (size_t)(eom - p) ? arg_sz : (size_t)(eom - p);
		memcpy(p, arg, copy_sz);
		p += copy_sz;
	}
	slog(ws->http_client->s, WEBDIS_DEBUG, log_msg, p - log_msg);
}

static void
ws_log_unauthorized(struct ws_client *ws) {
	if(!slog_enabled(ws->http_client->s, WEBDIS_DEBUG)) {
		return;
	}
	const char msg[] = "WS: 403";
	slog(ws->http_client->s, WEBDIS_DEBUG, msg, sizeof(msg)-1);
}


static int
ws_execute(struct ws_client *ws, struct ws_msg *msg) {

	struct http_client *c = ws->http_client;
	struct cmd*(*fun_extract)(struct http_client *, const char *, size_t) = NULL;
	formatting_fun fun_reply = NULL;
	ws_error_fun fun_error = NULL;

	if((c->path_sz == 1 && strncmp(c->path, "/", 1) == 0) ||
	   strncmp(c->path, "/.json", 6) == 0) {
		fun_extract = json_ws_extract;
		fun_reply = json_reply;
		fun_error = json_ws_error;
	} else if(strncmp(c->path, "/.raw", 5) == 0) {
		fun_extract = raw_ws_extract;
		fun_reply = raw_reply;
		fun_error = raw_ws_error;
	}

	if(fun_extract) {

		/* Parse websocket frame into a cmd object. */
		struct cmd *cmd = fun_extract(c, msg->payload, msg->payload_sz);

		if(cmd) {
			cmd->is_websocket = 1;

			if(ws->cmd != NULL) {
				/* This client already has its own connection to Redis
				   from a previous command; use it from now on. */

				/* free args for the previous cmd */
				cmd_free_argv(ws->cmd);
				/* copy args from what we just parsed to the persistent command */
				ws->cmd->count = cmd->count;
				ws->cmd->argv = cmd->argv;
				ws->cmd->argv_len = cmd->argv_len;
				ws->cmd->pub_sub_client = c; /* mark as persistent, otherwise the Redis context will be freed */

				cmd->argv = NULL;
				cmd->argv_len = NULL;
				cmd->count = 0;
				cmd_free(cmd);

				cmd = ws->cmd; /* replace pointer since we're about to pass it to cmd_send */
			} else {
				/* copy client info into cmd. */
				cmd_setup(cmd, c);

				/* First WS command; use Redis context from WS client. */
				cmd->ac = ws->ac;
				ws->cmd = cmd;
				cmd->pub_sub_client = c;
			}

			int is_subscribe = cmd_is_subscribe_args(cmd);
			int is_unsubscribe = cmd_is_unsubscribe_args(cmd);

			/* check that the client is able to run this command */
			if(!acl_allow_command(cmd, c->s->cfg, c)) {
				const char forbidden[] = "Forbidden";
				size_t error_sz;
				char *error = fun_error(403, forbidden, sizeof(forbidden)-1, &error_sz);
				ws_frame_and_send_response(ws, WS_BINARY_FRAME, error, error_sz);
				free(error);
				/* similar to HTTP: log command first and then rejection, both with "WS: " prefix */
				ws_log_cmd(ws, cmd);
				ws_log_unauthorized(ws);
			} else if(ws->ran_subscribe && !is_subscribe && !is_unsubscribe) { /* disallow non-subscribe commands after a subscribe */
				char error_msg[] = "Command not allowed after subscribe";
				ws_frame_and_send_response(ws, WS_BINARY_FRAME, error_msg, sizeof(error_msg)-1);
			} else { /* log and execute */
				ws_log_cmd(ws, cmd);
				cmd_send(cmd, fun_reply);
				ws->ran_subscribe = is_subscribe;
			}

			return 0;
		}
	}

	return -1;
}

static struct ws_msg *
ws_msg_new(enum ws_frame_type frame_type) {
	struct ws_msg *msg = calloc(1, sizeof(struct ws_msg));
	if(!msg) {
		return NULL;
	}
	msg->type = frame_type;
	return msg;
}

static int
ws_msg_add(struct ws_msg *m, const char *p, size_t psz, const unsigned char *mask) {

	/* add data to frame */
	size_t i;
	m->payload = realloc(m->payload, m->payload_sz + psz);
	if(!m->payload) {
		return -1;
	}
	memcpy(m->payload + m->payload_sz, p, psz);

	/* apply mask */
	for(i = 0; i < psz && mask; ++i) {
		m->payload[m->payload_sz + i] = (unsigned char)p[i] ^ mask[i%4];
	}

	/* save new size */
	m->payload_sz += psz;
	return 0;
}

static void
ws_msg_free(struct ws_msg *m) {

	free(m->payload);
	free(m);
}

/* checks to see if we have a complete message */
static enum ws_state
ws_peek_data(struct ws_client *ws, struct ws_msg **out_msg) {

	int has_mask;
	uint64_t len;
	const char *p;
	char *frame;
	unsigned char mask[4];
	char fin_bit_set;
	enum ws_frame_type frame_type;

	/* parse frame and extract contents */
	size_t sz = evbuffer_get_length(ws->rbuf);
	if(sz < 2) {
		return WS_READING; /* need more data */
	}
	/* copy into "frame" to process it */
	frame = malloc(sz);
	if(!frame) {
		return WS_ERROR;
	}
	int rem_ret = evbuffer_copyout(ws->rbuf, frame, sz); /* copy into frame but keep in rbuf */
	if(rem_ret < 0) {
		free(frame);
		return WS_ERROR;
	}

	fin_bit_set = frame[0] & 0x80 ? 1 : 0;
	frame_type = frame[0] & 0x0F;	/* lower 4 bits of first byte */
	has_mask = frame[1] & 0x80 ? 1:0;

	if(!has_mask) {
		/* a client MUST mask all frames that it sends to the server (RFC6455, 5.1. Overview) */
		ws->close_after_events = 1;
		const char close_code_reason[] = "\x03\xeaReceived a frame without a mask from the client (violates RFC6455, 5.1. Overview)."; /* 0x03,0xEA = 1002 - protocol error */
		ws_frame_and_send_response(ws, WS_CONNECTION_CLOSE, close_code_reason, sizeof(close_code_reason)-1);
		free(frame);
		return WS_ERROR;
	}

	/* get payload length */
	len = frame[1] & 0x7f;	/* remove leftmost bit */

	/* checking that the copyout frame contains the minimum data needed to determine the true length and mask in next step */
	size_t min_sz = 6; /* 2 bytes for flags and opcode + 4 bytes for mask */
	if(len == 126) {
		min_sz += sizeof(uint16_t);
	} else if(len == 127) {
		min_sz += sizeof(uint64_t);
	}
	if(sz < min_sz) { /* not enough data */
		free(frame);
		return WS_READING;
	}

	/* determine payload size (RFC 6455, section 5.2) */
	if(len <= 125) { /* data starts right after the mask */
		p = frame + 6;
		memcpy(&mask, frame + 2, sizeof(mask));
	} else if(len == 126) { /* size is stored in 16 bits after mask */
		uint16_t sz16;
		memcpy(&sz16, frame + 2, sizeof(uint16_t));
		len = ntohs(sz16);
		p = frame + 6 + sizeof(uint16_t);
		memcpy(&mask, frame + 4, sizeof(mask));
	} else if(len == 127) { /* size is stored in 64 bits after mask */
		uint64_t sz64 = *((uint64_t*)(frame+2));
		len = webdis_ntohll(sz64);
		p = frame + 6 + sizeof(uint64_t);
		memcpy(&mask, frame + 10, sizeof(mask));
	} else {
		free(frame);
		return WS_ERROR;
	}

	/* we now have the masked data starting in p, and its length.  */
	if(len > sz - (p - frame)) { /* not enough data */
		free(frame);
		return WS_READING;
	}

	int ev_copy = 0;
	if(out_msg) { /* we're extracting the message */
		struct ws_msg *msg = ws_msg_new(frame_type);
		if(!msg) {
			free(frame);
			return WS_ERROR;
		}
		*out_msg = msg; /* attach for it to be freed by caller */

		/* create new ws_msg object holding what we read */
		int add_ret = ws_msg_add(msg, p, len, mask);
		if(add_ret < 0) {
			free(frame);
			return WS_ERROR;
		}

		size_t processed_sz = len + (p - frame); /* length of data + header bytes between frame start and payload */
		msg->total_sz += processed_sz;
		ev_copy = evbuffer_drain(ws->rbuf, processed_sz); /* remove processed data from evbuffer */
	}
	free(frame);

	if(ev_copy < 0) {
		return WS_ERROR;
	} else if(fin_bit_set) {
		return WS_MSG_COMPLETE;
	} else {
		return WS_READING;	/* need more data */
	}
}

/**
 * Process some data just received on the socket.
 * Returns the number of messages processed in out_processed, if non-NULL.
 */
enum ws_state
ws_process_read_data(struct ws_client *ws, unsigned int *out_processed) {

	enum ws_state state;
	if(out_processed) *out_processed = 0;

	state = ws_peek_data(ws, NULL); /* check for complete message */

	while(state == WS_MSG_COMPLETE) {
		int ret = 0;
		struct ws_msg *msg = NULL;
		ws_peek_data(ws, &msg); /* extract message */

		if(msg && (msg->type == WS_TEXT_FRAME || msg->type == WS_BINARY_FRAME)) {
			ret = ws_execute(ws, msg);
			if(out_processed) (*out_processed)++;
		} else if(msg && msg->type == WS_PING) { /* respond to ping */
			ws_frame_and_send_response(ws, WS_PONG, msg->payload, msg->payload_sz);
		} else if(msg && msg->type == WS_CONNECTION_CLOSE) { /* respond to close frame */
			ws->close_after_events = 1;
			ws_frame_and_send_response(ws, WS_CONNECTION_CLOSE, msg->payload, msg->payload_sz);
		} else if(msg) {
			char format[] =  "Received unexpected WS frame type: 0x%x";
			char error[(sizeof format)];
			int error_len = snprintf(error, sizeof(error), format, msg->type);
			slog(ws->http_client->s, WEBDIS_WARNING, error, error_len);
		}

		/* free frame */
		if(msg) ws_msg_free(msg);

		if(ret != 0) {
			/* can't process frame. */
			slog(ws->http_client->s, WEBDIS_DEBUG, "ws_process_read_data: ws_execute failed", 0);
			return WS_ERROR;
		}
		state = ws_peek_data(ws, NULL);
	}
	return state;
}

int
ws_frame_and_send_response(struct ws_client *ws, enum ws_frame_type frame_type, const char *p, size_t sz) {

	/* we can have as much as 14 bytes in the header:
	 *   1 byte for 4 flag bits + 4 frame type bits
	 *   1 byte for the payload length indicator
	 *   8 bytes for the size of the payload (at most)
	 *   4 bytes for the masking key (if present)
	 */
	char *frame = malloc(sz + 14); /* create frame by prepending header */
	size_t frame_sz = 0;
	if(frame == NULL)
		return -1;

	/*
      The length of the "Payload data", in bytes: if 0-125, that is the
      payload length.  If 126, the following 2 bytes interpreted as a
      16-bit unsigned integer are the payload length.  If 127, the
      following 8 bytes interpreted as a 64-bit unsigned integer (the
      most significant bit MUST be 0) are the payload length.
	  */
	frame[0] = 0x80 | frame_type; /* frame type + EOM bit */
	if(sz <= 125) {
		frame[1] = sz;
		memcpy(frame + 2, p, sz);
		frame_sz = sz + 2;
	} else if(sz <= 65536) {
		uint16_t sz16 = htons(sz);
		frame[1] = 126;
		memcpy(frame + 2, &sz16, 2);
		memcpy(frame + 4, p, sz);
		frame_sz = sz + 4;
	} else { /* sz > 65536 */
		uint64_t sz_be = webdis_htonll(sz); /* big endian */
		char sz64[8];
		memcpy(sz64, &sz_be, 8);
		frame[1] = 127;
		memcpy(frame + 2, sz64, 8);
		memcpy(frame + 10, p, sz);
		frame_sz = sz + 10;
	}

	/* mark as keep alive, otherwise we'll close the connection after the first reply */
	int add_ret = evbuffer_add(ws->wbuf, frame, frame_sz);
	free(frame); /* no longer needed once added to buffer */
	if(add_ret < 0) {
		slog(ws->http_client->w->s, WEBDIS_ERROR, "Failed response allocation in ws_frame_and_send_response", 0);
		return -1;
	}

	/* send WS frame */
	return ws_schedule_write(ws);
}

void
ws_close_if_able(struct ws_client *ws) {

	ws->close_after_events = 1; /* note that we're closing */
	if(ws->scheduled_read || ws->scheduled_write) {
		return; /* still waiting for these events to trigger */
	}
	ws_client_free(ws); /* will close the socket */
}

static void
ws_can_read(int fd, short event, void *p) {

	int ret;
	struct ws_client *ws = p;
	(void)event;

	/* read pending data */
	ws->scheduled_read = 0;
	ret = evbuffer_read(ws->rbuf, fd, 4096);

	if(ret <= 0 || ws->close_after_events) {
		ws_close_if_able(ws); /* will close the socket once all events have triggered */
	} else {
		enum ws_state state = ws_process_read_data(ws, NULL);
		if(state == WS_READING) { /* need more data, schedule new read */
			ws_monitor_input(ws);
		} else if(state == WS_ERROR) {
			ws_close_if_able(ws);
		}
	}
}


static void
ws_can_write(int fd, short event, void *p) {

	int ret;
	struct ws_client *ws = p;
	(void)event;

	ws->scheduled_write = 0;

	/* send pending data */
	ret = evbuffer_write_atmost(ws->wbuf, fd, 4096);

	if(ret <= 0) {
		ws_close_if_able(ws); /* will close the socket once all events have triggered */
	} else {
		if(evbuffer_get_length(ws->wbuf) > 0) { /* more data to send */
			ws_schedule_write(ws);
		} else if(ws->close_after_events) { /* we're done! */
			ws_close_if_able(ws);
		} else {
			/* check if we can read more data */
			unsigned int processed = 0;
			ws_process_read_data(ws, &processed); /* process any pending data we've already read */
			ws_monitor_input(ws);				  /* let's read more from the client */
		}
	}
}

static int
ws_schedule_write(struct ws_client *ws) {
	struct http_client *c = ws->http_client;
	if(!ws->scheduled_write) {
		ws->scheduled_write = 1;
		return event_base_once(c->w->base, c->fd, EV_WRITE, ws_can_write, ws, NULL);
	}
	return 0;
}

int
ws_monitor_input(struct ws_client *ws) {
	struct http_client *c = ws->http_client;
	if(!ws->scheduled_read) {
		ws->scheduled_read = 1;
		return event_base_once(c->w->base, c->fd, EV_READ, ws_can_read, ws, NULL);
	}
	return 0;
}
