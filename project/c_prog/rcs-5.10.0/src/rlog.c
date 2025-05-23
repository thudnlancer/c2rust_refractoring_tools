/* Print log messages and other information about RCS files.

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995 Paul Eggert
   Copyright (C) 1982, 1988, 1989 Walter Tichy

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "base.h"
#include <string.h>
#include <stdlib.h>
#include <errno.h>           /* temporary, to support ‘read_positive_integer’,
                                before it and ‘compute_a_d’ move to grok.  */
#include "rlog.help"
#include "b-complain.h"
#include "b-divvy.h"
#include "b-esds.h"
#include "b-excwho.h"
#include "b-fb.h"
#include "b-fro.h"

static char const ks_delims[] = ", \t\n;";

struct revrange
{
  char const *beg;
  char const *end;
  int nfield;
};

struct daterange
{
  char beg[DATESIZE];
  char end[DATESIZE];
  bool open_end;
};

struct date_selection
{
  struct link *in;                      /* start - end */
  struct link *by;                      /* end only */
};

struct criteria
{
  struct link *revs, *actual;
  /* On the first pass (option processing), push onto ‘.revs’.
     After grokking, walk ‘.revs’ and push onto ‘.actual’.  */

  struct link *authors;
  struct link *lockers;
  struct link *states;
};

#define PUSH(x,ls)  ls = prepend (x, ls, PLEXUS)

static bool
tokenize (char *argv, struct link **chain)
/* Tokenize (destructively) ‘argv’ using ‘ks_delims’.
   Push tokens on ‘chain’ (LIFO).
   Return true if any tokens were parsed.  */
{
  struct link *before = *chain;
  char *s, *save, *token;

  for (s = argv;
       (token = strtok_r (s, ks_delims, &save));
       s = NULL)
    PUSH (token, *chain);

  return *chain != before;
}

static void
cleanup (int *exitstatus)
{
  if (FLOW (erroneous))
    *exitstatus = exit_failure;
  fro_zclose (&FLOW (from));
}

static void
getlocker (char *argv, struct criteria *criteria)
/* Parse logins from ‘argv’; store in ‘criteria->lockers’.  */
{
  tokenize (argv, &criteria->lockers);
}

/* TODO: Move into b-fro.c, since counting lines is not sensitive to
   "@@" presence (we can work in-place, avoiding ‘string_from_atat’).  */

static long
read_positive_integer (char const **p)
{
  long rv;
  char *end;

  errno = 0;
  if (1 > (rv = strtol (*p, &end, 10)))
    RFATAL ("non-positive integer");
  if (ERANGE == errno)
    RFATAL ("bad integer");
  *p = end;
  return rv;
}

static void
count_a_d (long *a, long *d, struct atat *edits)
{
  struct cbuf s = string_from_atat (SINGLE, edits);
  char const *end = s.string + s.size;
  long *totals = zlloc (SINGLE, 2 * sizeof (long));

  for (char const *p = s.string; p < end; p++)
    {
      bool add = ('a' == *p++);
      long count;

      /* Skip the line number.  */
      p = strchr (p, ' ');
      count = read_positive_integer (&p);

      totals[add] += count;
      if (add)
        /* Ignore the actual lines.  */
        while (count--)
          {
            size_t remaining = end - p;

            if (! (p = memchr (++p, '\n', remaining)))
              /* No final newline; skip out.  */
              goto done;
          }
    }
 done:
  *a = totals[1];
  *d = totals[0];
  brush_off (SINGLE, totals);
}

static void
putadelta (struct delta const *node, struct delta const *editscript,
           char const *insDelFormat)
/* Print delta ‘node’.  ‘editscript’ indicates where the editscript is
   stored; it equals ‘node’ if this node is not in trunk.  */
{
  FILE *out = stdout;
  char datebuf[FULLDATESIZE];
  bool pre5 = BE (version) < VERSION (5);
  struct atat *log;

  aprintf (out, "----------------------------\nrevision %s%s",
           node->num,
           pre5 ? "        " : "");
  if (node->lockedby)
    aprintf (out, pre5 + "\tlocked by: %s;", node->lockedby);

  aprintf (out, "\ndate: %s;  author: %s;  state: %s;",
           date2str (node->date, datebuf), node->author, node->state);

  if (editscript && editscript != REPO (tip))
    {
      bool trunk = node != editscript;
      long a, d;

      count_a_d (trunk ? &d : &a,
                 trunk ? &a : &d,
                 editscript->text);
      aprintf (out, insDelFormat, a, d);
    }

  if (node->branches)
    {
      aputs ("\nbranches:", out);
      for (struct wlink *ls = node->branches; ls; ls = ls->next)
        {
          struct delta *delta = ls->entry;

          aprintf (out, "  %s;", BRANCHNO (delta->num));
        }
    }

  if (node->commitid)
    aprintf (out, "%s commitid: %s",
             editscript ? ";" : "",
             node->commitid);

  newline (out);
  if ((log = node->log)
      && log->beg + 1 < ATAT_END (log))
    atat_display (out, log, true);
  else
    awrite (EMPTYLOG "\n", sizeof (EMPTYLOG), out);
}

static void
putrunk (const char *insDelFormat)
/* Print revisions chosen, which are in trunk.  */
{
  for (struct delta const *ptr = REPO (tip); ptr; ptr = ptr->ilk)
    if (ptr->selector)
      putadelta (ptr, ptr->ilk, insDelFormat);
}

static struct delta const *putforest (struct wlink const *, const char *);

static void
putree (struct delta const *root, const char *insDelFormat)
/* Print delta tree from ‘root’ (not including trunk)
   in reverse order on each branch.  */
{
  while (root)
    if (! root->branches)
      root = root->ilk;
    else
      {
        /* FIXME: This recurses deeply in the worst case.  */
        putree (root->ilk, insDelFormat);
        root = putforest (root->branches, insDelFormat);
      }
}

static void
putabranch (struct delta const *root, const char *insDelFormat)
/* Print one branch from ‘root’.  */
{
  while (!root->selector)
    {
      root = root->ilk;
      if (!root)
        return;
    }

  /* FIXME: This recurses deeply in the worst case.  */
  if (root->ilk)
    putabranch (root->ilk, insDelFormat);

  putadelta (root, root, insDelFormat);
}

static struct delta const *
putforest (struct wlink const *branchroot, const char *insDelFormat)
/* Print branches that have the same direct ancestor ‘branchroot’.  */
{
  /* FIXME: This recurses deeply in the worst case.  */
  if (branchroot->next)
    putforest (branchroot->next, insDelFormat);

  putabranch (branchroot->entry, insDelFormat);
  return branchroot->entry;
}

static bool
extractdelta (struct delta const *pdelta, bool lockflag,
              struct criteria *criteria)
/* Return true if ‘pdelta’ matches the selection critera.  */
{
  struct link const *pstate;
  struct link const *pauthor;
  int length;

  /* Only certain authors wanted.  */
  if ((pauthor = criteria->authors))
    while (STR_DIFF (pauthor->entry, pdelta->author))
      if (!(pauthor = pauthor->next))
        return false;
  /* Only certain states wanted.  */
  if ((pstate = criteria->states))
    while (STR_DIFF (pstate->entry, pdelta->state))
      if (!(pstate = pstate->next))
        return false;
  /* Only locked revisions wanted.  */
  if (lockflag && !lock_on (pdelta))
    return false;
  /* Only certain revs or branches wanted.  */
  for (struct link *ls = criteria->actual; ls;)
    {
      struct revrange const *rr = ls->entry;

      length = rr->nfield;
      if (countnumflds (pdelta->num) == length + ODDP (length)
          && 0 <= compartial (pdelta->num, rr->beg, length)
          && 0 <= compartial (rr->end, pdelta->num, length))
        break;
      if (! (ls = ls->next))
        return false;
    }
  return true;
}

static void
exttree (struct delta *root, bool lockflag,
         struct criteria *criteria)
/* Select revisions, starting with ‘root’.  */
{
  while (root)
    {
      root->selector = extractdelta (root, lockflag, criteria);
      root->pretty_log.string = NULL;

      if (! root->branches)
        root = root->ilk;
      else
        {
          /* FIXME: This recurses deeply in the worst case.  */
          struct wlink *ls;
          exttree (root->ilk, lockflag, criteria);
          for (ls = root->branches; ls->next; ls = ls->next)
            exttree (ls->entry, lockflag, criteria);
          root = ls->entry;
        }
    }
}

static void
getauthor (char *argv, struct criteria *criteria)
/* Parse logins from ‘argv’; store in ‘criteria->authors’.
   If none specified, default to the user login.  */
{
  if (! tokenize (argv, &criteria->authors))
    PUSH (getusername (false),
          criteria->authors);
}

static void
getstate (char *argv, struct criteria *criteria)
/* Parse states from ‘argv’; store in ‘criteria->states’.
   Signal error if none specified.  */
{
  if (! tokenize (argv, &criteria->states))
    PERR ("missing state attributes after -s option");
}

static void
trunclocks (struct criteria *criteria)
/* Truncate the list of locks to those that are held by the id's on
   ‘criteria->lockers’.  Do not truncate if ‘criteria->lockers’ empty.  */
{
  struct link const *plocker;
  struct link box, *tp;

  if (!criteria->lockers)
    return;

  /* Shorten locks to those contained in ‘criteria->lockers’.  */
  for (box.next = GROK (locks), tp = &box; tp->next;)
    {
      struct rcslock const *rl = tp->next->entry;

      for (plocker = criteria->lockers;;)
        if (STR_SAME (plocker->entry, rl->login))
          {
            tp = tp->next;
            break;
          }
        else if (!(plocker = plocker->next))
          {
            tp->next = tp->next->next;
            GROK (locks) = box.next;
            break;
          }
    }
}

static void
recentdate (struct delta const *root, struct daterange *r)
/* Find the delta that is closest to the cutoff date ‘pd’ among the
   revisions selected by ‘exttree’.  Successively narrow down the
   interval given by ‘pd’, and set the ‘strtdate’ of ‘pd’ to the date
   of the selected delta.  */
{
  while (root)
    {
      if (root->selector
          && !DATE_LT (root->date, r->beg)
          && !DATE_GT (root->date, r->end))
        {
          strncpy (r->beg, root->date, DATESIZE);
          r->beg[DATESIZE - 1] = '\0';
        }

      struct wlink *ls = root->branches;
      if (!ls)
        root = root->ilk;
      else
        {
          /* FIXME: This recurses deeply in the worst case.  */
          for (; ls->next; ls = ls->next)
            recentdate (ls->entry, r);
          root = ls->entry;
        }
    }
}

static size_t
extdate (struct delta *root, struct date_selection *datesel)
/* Select revisions which are in the date range specified in ‘datesel->by’
   and ‘datesel->in’, starting at ‘root’.  Return number of revisions
   selected, including those already selected.  */
{
  size_t revno = 0;

  for (; root; root = root->ilk)
    {
      if (datesel->in || datesel->by)
        {
          struct daterange const *r;
          bool open_end, sel = false;

          for (struct link *ls = datesel->in; ls; ls = ls->next)
            {
              r = ls->entry;
              open_end = r->open_end;
              if ((sel = ((!r->beg[0]
                           || (open_end
                               ? DATE_LT (r->beg, root->date)
                               : !DATE_GT (r->beg, root->date)))
                          &&
                          (!r->end[0]
                           || (open_end
                               ? DATE_LT (root->date, r->end)
                               : !DATE_GT (root->date, r->end))))))
                break;
            }
          if (!sel)
            {
              for (struct link *ls = datesel->by; ls; ls = ls->next)
                {
                  r = ls->entry;
                  if ((sel = DATE_EQ (root->date, r->beg)))
                    break;
                }
              if (!sel)
                root->selector = false;
            }
        }

      revno += root->selector;

      /* FIXME: This recurses deeply in the worst case.  */
      for (struct wlink *ls = root->branches; ls; ls = ls->next)
        revno += extdate (ls->entry, datesel);
    }

  return revno;
}

static void
getdatepair (char *argv, struct date_selection *datesel)
/* Get time range from command line and store in ‘datesel->in’ if
   a time range specified or in ‘datesel->by’ if a time spot specified.  */
{
  register char c;
  struct daterange *r;
  char const *rawdate;
  bool switchflag;

  argv--;
  while ((c = *++argv) == ',' || c == ' ' || c == '\t' || c == '\n'
         || c == ';')
    continue;
  if (c == '\0')
    {
      PERR ("missing date/time after -d");
      return;
    }

  while (c != '\0')
    {
      switchflag = false;
      r = ZLLOC (1, struct daterange);
      if (c == '<')                     /* <DATE */
        {
          c = *++argv;
          if (!(r->open_end = c != '='))
            c = *++argv;
          r->beg[0] = '\0';
        }
      else if (c == '>')                /* >DATE */
        {
          c = *++argv;
          if (!(r->open_end = c != '='))
            c = *++argv;
          r->end[0] = '\0';
          switchflag = true;
        }
      else
        {
          rawdate = argv;
          while (c != '<' && c != '>' && c != ';' && c != '\0')
            c = *++argv;
          *argv = '\0';
          if (c == '>')
            switchflag = true;
          str2date (rawdate, switchflag ? r->end : r->beg);
          if (c == ';' || c == '\0')    /* DATE */
            {
              memcpy (r->end, r->beg, DATESIZE);
              PUSH (r, datesel->by);
              goto end;
            }
          else                   /* DATE< or DATE> (see ‘switchflag’) */
            {
              bool eq = argv[1] == '=';

              r->open_end = !eq;
              argv += eq;
              while ((c = *++argv) == ' ' || c == '\t' || c == '\n')
                continue;
              if (c == ';' || c == '\0')
                {
                  /* Second date missing.  */
                  (switchflag ? r->beg : r->end)[0] = '\0';
                  PUSH (r, datesel->in);
                  goto end;
                }
            }
        }
      rawdate = argv;
      while (c != '>' && c != '<' && c != ';' && c != '\0')
        c = *++argv;
      *argv = '\0';
      str2date (rawdate, switchflag ? r->beg : r->end);
      PUSH (r, datesel->in);
    end:
      if (BE (version) < VERSION (5))
        r->open_end = false;
      if (c == '\0')
        return;
      while ((c = *++argv) == ';' || c == ' ' || c == '\t' || c == '\n')
        continue;
    }
}

static bool
checkrevpair (char const *num1, char const *num2)
/* Check whether ‘num1’, ‘num2’ are a legal pair, i.e.
   only the last field differs and have same number of
   fields (if length <= 2, may be different if first field).  */
{
  int length = countnumflds (num1);

  if (countnumflds (num2) != length
      || (2 < length && compartial (num1, num2, length - 1) != 0))
    {
      RERR ("invalid branch or revision pair %s : %s", num1, num2);
      return false;
    }
  return true;
}

#define ZERODATE  "0.0.0.0.0.0"

static bool
getnumericrev (bool branchflag, struct criteria *criteria)
/* Get the numeric name of revisions stored in ‘criteria->revs’; store
   them in ‘criteria->actual’.  If ‘branchflag’, also add default branch.  */
{
  struct link *ls;
  struct revrange *rr;
  int n;
  struct cbuf s, e;
  char const *lrev;
  struct cbuf const *rstart, *rend;
  struct delta *tip = REPO (tip);
  char const *defbr = GROK (branch);

  criteria->actual = NULL;
  for (ls = criteria->revs; ls; ls = ls->next)
    {
      struct revrange const *from = ls->entry;

      n = 0;
      rstart = &s;
      rend = &e;

      switch (from->nfield)
        {

        case 1:                         /* -rREV */
          if (!fully_numeric_no_k (&s, from->beg))
            goto freebufs;
          rend = &s;
          n = countnumflds (s.string);
          if (!n && (lrev = tiprev ()))
            {
              s.string = lrev;
              n = countnumflds (lrev);
            }
          break;

        case 2:                         /* -rREV: */
          if (!fully_numeric_no_k (&s, from->beg))
            goto freebufs;
          e.string = (2 > (n = countnumflds (s.string)))
            ? ""
            : SHSNIP (&e.size, s.string, strrchr (s.string, '.'));
          break;

        case 3:                         /* -r:REV */
          if (!fully_numeric_no_k (&e, from->end))
            goto freebufs;
          if ((n = countnumflds (e.string)) < 2)
            s.string = ".0";
          else
            {
              SHACCR (e.string, strrchr (e.string, '.'));
              accf (PLEXUS, ".0");
              s.string = SHSTR (&s.size);
            }
          break;

        default:                        /* -rREV1:REV2 */
          if (!(fully_numeric_no_k (&s, from->beg)
                && fully_numeric_no_k (&e, from->end)
                && checkrevpair (s.string, e.string)))
            goto freebufs;
          n = countnumflds (s.string);
          /* Swap if out of order.  */
          if (compartial (s.string, e.string, n) > 0)
            {
              rstart = &e;
              rend = &s;
            }
          break;
        }

      if (n)
        {
          rr = FALLOC (struct revrange);
          rr->nfield = n;
          rr->beg = rstart->string;
          rr->end = rend->string;
          PUSH (rr, criteria->actual);
        }
    }
  /* Now take care of ‘branchflag’.  */
  if (branchflag && (defbr || tip))
    {
      rr = FALLOC (struct revrange);
      rr->beg = rr->end = defbr
        ? defbr
        : TAKE (1, tip->num);
      rr->nfield = countnumflds (rr->beg);
      PUSH (rr, criteria->actual);
    }

freebufs:
  return !ls;
}

static void
putrevpairs (char const *b, char const *e, bool sawsep, void *data)
/* Store a revision or branch range into ‘creteria->revs’.  */
{
  struct criteria *criteria = data;
  struct revrange *rr = ZLLOC (1, struct revrange);

  rr->beg = b;
  rr->end = e;
  rr->nfield = (!sawsep
                ? 1                     /* -rREV */
                : (!e[0]
                   ? 2                  /* -rREV: */
                   : (!b[0]
                      ? 3               /* -r:REV */
                      : 4)));           /* -rREV1:REV2 */
  PUSH (rr, criteria->revs);
}

DECLARE_PROGRAM (rlog, TYAG_IMMEDIATE);

static int
rlog_main (const char *cmd, int argc, char **argv)
{
  int exitstatus = EXIT_SUCCESS;
  bool branchflag = false;
  bool lockflag = false;
  struct date_selection datesel = { .in = NULL, .by = NULL };
  struct criteria criteria =
    {
      .revs = NULL,
      .authors = NULL,
      .lockers = NULL,
      .states = NULL
    };
  char const *insDelFormat;
  FILE *out;
  char *a, **newargv;
  char const *accessListString, *accessFormat;
  char const *headFormat, *symbolFormat;
  bool descflag, selectflag;
  bool onlylockflag;                   /* print only files with locks */
  bool onlyRCSflag;                    /* print only RCS filename */
  bool pre5;
  bool shownames;
  size_t revno;

  CHECK_HV (cmd);
  gnurcs_init (&program);

  descflag = selectflag = shownames = true;
  onlylockflag = onlyRCSflag = false;
  out = stdout;

  argc = getRCSINIT (argc, argv, &newargv);
  argv = newargv;
  while (a = *++argv, 0 < --argc && *a++ == '-')
    {
      switch (*a++)
        {

        case 'L':
          onlylockflag = true;
          break;

        case 'N':
          shownames = false;
          break;

        case 'R':
          onlyRCSflag = true;
          break;

        case 'l':
          lockflag = true;
          getlocker (a, &criteria);
          break;

        case 'b':
          branchflag = true;
          break;

        case 'r':
          parse_revpairs ('r', a, &criteria, putrevpairs);
          break;

        case 'd':
          getdatepair (a, &datesel);
          break;

        case 's':
          getstate (a, &criteria);
          break;

        case 'w':
          getauthor (a, &criteria);
          break;

        case 'h':
          descflag = false;
          break;

        case 't':
          selectflag = false;
          break;

        case 'q':
          /* This has no effect; it's here for consistency.  */
          BE (quiet) = true;
          break;

        case 'x':
          BE (pe) = a;
          break;

        case 'z':
          zone_set (a);
          break;

        case 'T':
          /* Ignore -T, so that RCSINIT can contain -T.  */
          if (*a)
            goto unknown;
          break;

        case 'V':
          setRCSversion (*argv);
          break;

        default:
        unknown:
          bad_option (*argv);
        };
    }
  /* (End of option processing.)  */

  if (!(descflag | selectflag))
    {
      PWARN ("-t overrides -h.");
      descflag = true;
    }

  pre5 = BE (version) < VERSION (5);
  if (pre5)
    {
      accessListString = "\naccess list:   ";
      accessFormat = "  %s";
      headFormat =
        "\nRCS file:        %s;   Working file:    %s\nhead:           %s%s\nbranch:         %s%s\nlocks:         ";
      insDelFormat = "  lines added/del: %ld/%ld";
      symbolFormat = "  %s: %s;";
    }
  else
    {
      accessListString = "\naccess list:";
      accessFormat = "\n\t%s";
      headFormat =
        "\nRCS file: %s\nWorking file: %s\nhead:%s%s\nbranch:%s%s\nlocks:%s";
      insDelFormat = "  lines: +%ld -%ld";
      symbolFormat = "\n\t%s: %s";
    }

  /* Now handle all filenames.  */
  if (FLOW (erroneous))
    cleanup (&exitstatus);
  else if (argc < 1)
    PFATAL ("no input file");
  else
    for (; 0 < argc; cleanup (&exitstatus), ++argv, --argc)
      {
        char const *repo_filename;
        struct delta *tip;
        char const *defbr;
        bool strictly_locking;
        int kws;
        struct link *locks;

        ffree ();

        if (pairnames (argc, argv, rcsreadopen, true, false) <= 0)
          continue;

        /* ‘REPO (filename)’ contains the name of the RCS file,
           and ‘FLOW (from)’ the file descriptor;
           ‘MANI (filename)’ contains the name of the working file.  */
        repo_filename = REPO (filename);
        tip = REPO (tip);
        defbr = GROK (branch);
        locks = GROK (locks);
        strictly_locking = BE (strictly_locking);
        kws = BE (kws);

        /* Keep only those locks given by ‘-l’.  */
        if (lockflag)
          trunclocks (&criteria);

        /* Do nothing if ‘-L’ is given and there are no locks.  */
        if (onlylockflag && !locks)
          continue;

        if (onlyRCSflag)
          {
            aprintf (out, "%s\n", repo_filename);
            continue;
          }

        if (!getnumericrev (branchflag, &criteria))
          continue;

        /* Print RCS filename, working filename and optional
           administrative information.  Could use ‘getfullRCSname’
           here, but that is very slow.  */
        aprintf (out, headFormat, repo_filename, MANI (filename),
                 tip ? " " : "",
                 tip ? tip->num : "",
                 defbr ? " " : "",
                 defbr ? defbr : "",
                 strictly_locking ? " strict" : "");
        format_locks (out, symbolFormat);
        if (strictly_locking && pre5)
          aputs ("  ;  strict" + (locks ? 3 : 0), out);

        /* Print access list.  */
        aputs (accessListString, out);
        for (struct link *ls = GROK (access); ls; ls = ls->next)
          aprintf (out, accessFormat, ls->entry);

        if (shownames)
          {
            /* Print symbolic names.  */
            aputs ("\nsymbolic names:", out);
            format_assocs (out, symbolFormat);
          }
        if (pre5)
          {
            aputs ("\ncomment leader:  \"", out);
            awrite (REPO (log_lead).string, REPO (log_lead).size, out);
            afputc ('\"', out);
          }
        if (!pre5 || kws != kwsub_kv)
          aprintf (out, "\nkeyword substitution: %s", kwsub_string (kws));

        aprintf (out, "\ntotal revisions: %zu", GROK (deltas_count));

        revno = 0;

        if (tip && selectflag & descflag)
          {
            exttree (tip, lockflag, &criteria);

            /* Get most recently date of the dates pointed by ‘duelst’.  */
            for (struct link *ls = datesel.by; ls; ls = ls->next)
              {
                struct daterange const *incomplete = ls->entry;
                struct daterange *r = ZLLOC (1, struct daterange);

                /* Couldn't this have been done before?  --ttn  */
                *r = *incomplete;
                memcpy (r->beg, ZERODATE, sizeof ZERODATE);
                ls->entry = r;
                recentdate (tip, r);
              }

            revno = extdate (tip, &datesel);

            aprintf (out, ";\tselected revisions: %zu", revno);
          }

        newline (out);
        if (descflag)
          {
            struct atat *desc = GROK (desc);

            aputs ("description:\n", out);
            atat_display (out, desc, true);
          }
        if (revno)
          {
            putrunk (insDelFormat);
            putree (tip, insDelFormat);
          }
        aputs (equal_line, out);
      }
  Ozclose (&out);
  gnurcs_goodbye ();
  return exitstatus;
}

static const uint8_t rlog_aka[10] =
{
  2 /* count */,
  3,'l','o','g',
  4,'r','l','o','g'
};

YET_ANOTHER_COMMAND (rlog);

/*:help
[options] file ...
Options:
  -L            Ignore RCS files with no locks set.
  -R            Print the RCS file name only.
  -h            Print only the "header" information.
  -t            Like -h, but also include the description.
  -N            Omit symbolic names.
  -b            Select the default branch.
  -dDATES       Select revisions in the range DATES, with spec:
                  D      -- single revision D or earlier
                  D1<D2  -- between D1 and D2, exclusive
                  D2>D1  -- likewise
                  <D, D> -- before D
                  >D, D< -- after D
                Use <= or >= to make ranges inclusive; DATES
                may also be a list of semicolon-separated specs.
  -l[WHO]       Select revisions locked by WHO (comma-separated list)
                only, or by anyone if WHO is omitted.
  -r[REVS]      Select revisions in REVS, a comma-separated list of
                range specs, one of: REV, REV:, :REV, REV1:REV2
  -sSTATES      Select revisions with state in STATES (comma-separated list).
  -w[WHO]       Select revisions checked in by WHO (comma-separated list),
                or by the user if WHO is omitted.
  -T            No effect; included for compatibility with other commands.
  -V            Obsolete; do not use.
  -VN           Emulate RCS version N.
  -xSUFF        Specify SUFF as a slash-separated list of suffixes
                used to identify RCS file names.
  -zZONE        Specify date output format in keyword-substitution.
  -q            No effect, included for consistency with other commands.
*/

/* rlog.c ends here */
