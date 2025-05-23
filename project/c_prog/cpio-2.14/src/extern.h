/* extern.h - External declarations for cpio.  Requires system.h.
   Copyright (C) 1990-2023 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public
   License along with this program; if not, write to the Free
   Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
   Boston, MA 02110-1301 USA.  */

#include "paxlib.h"
#include "quotearg.h"
#include "quote.h"
#include "inttostr.h"

enum archive_format
{
  arf_unknown, arf_binary, arf_oldascii, arf_newascii, arf_crcascii,
  arf_tar, arf_ustar, arf_hpoldascii, arf_hpbinary
};

extern enum archive_format archive_format;
extern int reset_time_flag;
extern int io_block_size;
extern int create_dir_flag;
extern int rename_flag;
extern char *rename_batch_file;
extern int table_flag;
extern int unconditional_flag;
extern int verbose_flag;
extern int dot_flag;
extern int link_flag;
extern int retain_time_flag;
extern int crc_i_flag;
extern int append_flag;
extern int swap_bytes_flag;
extern int swap_halfwords_flag;
extern int swapping_bytes;
extern int swapping_halfwords;
extern int set_owner_flag;
extern uid_t set_owner;
extern int set_group_flag;
extern gid_t set_group;
extern int no_chown_flag;
extern int sparse_flag;
extern int quiet_flag;
extern int only_verify_crc_flag;
extern int no_abs_paths_flag;
extern unsigned int warn_option;
extern mode_t newdir_umask;
extern int renumber_inodes_option;
extern int ignore_devno_option;
extern int ignore_dirnlink_option;

/* Values for warn_option */
#define CPIO_WARN_NONE     0
#define CPIO_WARN_TRUNCATE 0x01
#define CPIO_WARN_INTERDIR 0x02
#define CPIO_WARN_ALL      (unsigned int)-1

extern bool to_stdout_option;

extern off_t last_header_start;
extern int copy_matching_files;
extern int numeric_uid;
extern char *pattern_file_name;
extern char *new_media_message;
extern char *new_media_message_with_number;
extern char *new_media_message_after_number;
extern int archive_des;
extern char *archive_name;
extern char *rsh_command_option;
extern uint32_t crc;
#ifdef DEBUG_CPIO
extern int debug_flag;
#endif

extern char *input_buffer, *output_buffer;
extern char *in_buff, *out_buff;
extern size_t input_buffer_size;
extern size_t input_size, output_size;
extern off_t input_bytes, output_bytes;

extern char *directory_name;
extern char **save_patterns;
extern int num_patterns;
extern char name_end;
extern char input_is_special;
extern char output_is_special;
extern char input_is_seekable;
extern char output_is_seekable;
extern int (*xstat) ();
extern void (*copy_function) ();
extern char *change_directory_option;

#define STRINGIFY_BIGINT(i, b) umaxtostr (i, b)
enum { UINTMAX_STRSIZE_BOUND = INT_BUFSIZE_BOUND (intmax_t) };



/* copyin.c */
void warn_junk_bytes (long bytes_skipped);
/* FIXME: make read_* static in copyin.c */
void read_in_header (struct cpio_file_stat *file_hdr, int in_des);
void read_in_old_ascii (struct cpio_file_stat *file_hdr, int in_des);
void read_in_new_ascii (struct cpio_file_stat *file_hdr, int in_des);
void read_in_binary (struct cpio_file_stat *file_hdr,
		     struct old_cpio_header *short_hdr, int in_des);
void swab_array (char *arg, int count);
void process_copy_in (void);
void long_format (struct cpio_file_stat *file_hdr, char const *link_name);

/* copyout.c */
int write_out_header (struct cpio_file_stat *file_hdr, int out_des);
void process_copy_out (void);
int to_ascii (char *where, uintmax_t v, size_t digits, unsigned logbase,
	      bool nul);
void field_width_error (const char *filename, const char *fieldname,
			uintmax_t value, size_t width, bool nul);

/* copypass.c */
void process_copy_pass (void);
int link_to_maj_min_ino (char *file_name, int st_dev_maj,
			 int st_dev_min, ino_t st_ino);
int link_to_name (char const *link_name, char const *link_target);

/* dirname.c */
char *dirname (char *path);

/* filemode.c */
void mode_string (unsigned int mode, char *str);

/* idcache.c */
char *getgroup (gid_t gid);
char *getuser (uid_t uid);
uid_t *getuidbyname (char *user);
gid_t *getgidbyname (char *group);

/* main.c */
void process_args (int argc, char *argv[]);
void initialize_buffers (void);

/* makepath.c */
int make_path (char const *argpath, uid_t owner, gid_t group,
	       const char *verbose_fmt_string);

/* tar.c */
int write_out_tar_header (struct cpio_file_stat *file_hdr, int out_des);
int null_block (long *block, int size);
void read_in_tar_header (struct cpio_file_stat *file_hdr, int in_des);
int otoa (char *s, unsigned long *n);
int is_tar_header (char *buf);
int is_tar_filename_too_long (char *name);

/* userspec.c */
const char *parse_user_spec (const char *spec_arg, uid_t *uid, gid_t *gid,
			     char **username_arg, char **groupname_arg);

/* util.c */
void tape_empty_output_buffer (int out_des);
void disk_empty_output_buffer (int out_des, bool flush);
void swahw_array (char *ptr, int count);
void tape_buffered_write (char *in_buf, int out_des, off_t num_bytes);
void tape_buffered_read (char *in_buf, int in_des, off_t num_bytes);
int tape_buffered_peek (char *peek_buf, int in_des, int num_bytes);
void tape_toss_input (int in_des, off_t num_bytes);
void copy_files_tape_to_disk (int in_des, int out_des, off_t num_bytes);
void copy_files_disk_to_tape (int in_des, int out_des, off_t num_bytes, char *filename);
void copy_files_disk_to_disk (int in_des, int out_des, off_t num_bytes, char *filename);
void warn_if_file_changed (char *file_name, off_t old_file_size,
			   time_t old_file_mtime);
void create_all_directories (char const *name);
void prepare_append (int out_file_des);
char *find_inode_file (ino_t node_num,
		       unsigned long major_num, unsigned long minor_num);
struct inode_val *add_inode (ino_t node_num, char *file_name,
			     unsigned long major_num, unsigned long minor_num);
int open_archive (char *file);
void tape_offline (int tape_des);
void get_next_reel (int tape_des);
void set_new_media_message (char *message);
#ifdef HPUX_CDF
char *add_cdf_double_slashes (char *filename);
#endif
void write_nuls_to_file (off_t num_bytes, int out_des,
			 void (*writer) (char *in_buf,
					 int out_des, off_t num_bytes));
#define DISK_IO_BLOCK_SIZE	512

/* FIXME: Move to system.h? */
#ifndef SYMLINK_USES_UMASK
# define UMASKED_SYMLINK(name1,name2,mode)    symlink(name1,name2)
#else
# define UMASKED_SYMLINK(name1,name2,mode)    umasked_symlink(name1,name2,mode)
#endif /* SYMLINK_USES_UMASK */

void set_perms (int fd, struct cpio_file_stat *header);
void set_file_times (int fd, const char *name, unsigned long atime,
		     unsigned long mtime);
void stat_to_cpio (struct cpio_file_stat *hdr, struct stat *st);
void cpio_to_stat (struct stat *st, struct cpio_file_stat *hdr);
void cpio_safer_name_suffix (char *name, bool link_target,
			     bool absolute_names, bool strip_leading_dots);
int cpio_create_dir (struct cpio_file_stat *file_hdr, int existing_dir);
void change_dir (void);

/* FIXME: The following three should be defined in paxutils */
#define LG_8  3
#define LG_16 4
/* The maximum uintmax_t value that can be represented with DIGITS digits,
   assuming that each digit is BITS_PER_DIGIT wide.  */
#define MAX_VAL_WITH_DIGITS(digits, bits_per_digit) \
   ((digits) * (bits_per_digit) < sizeof (uintmax_t) * CHAR_BIT \
    ? ((uintmax_t) 1 << ((digits) * (bits_per_digit))) - 1 \
    : (uintmax_t) -1)


uintmax_t from_ascii (char const *where, size_t digs, unsigned logbase);

#define FROM_OCTAL(f) from_ascii (f, sizeof f, LG_8)
#define FROM_HEX(f) from_ascii (f, sizeof f, LG_16)

void delay_cpio_set_stat (struct cpio_file_stat *file_stat,
			  mode_t invert_permissions);
void delay_set_stat (char const *file_name, struct stat *st,
		     mode_t invert_permissions);
int repair_delayed_set_stat (struct cpio_file_stat *file_hdr);
void apply_delayed_set_stat (void);

int arf_stores_inode_p (enum archive_format arf);
