+++
title = "Eliminate Unintended Aliases"
description = "Some nice git config"
+++

Over the life of a `git` repository, a single committer can end up committing
to the repository under different aliases.  This can happen for a variety of
reasons:

- The author may have changed their real name intentionally
- Author may have changed 

For me personally, this has usually happened unintentionally.  An annoying ambivalence or inattentiveness for the name or email address I"m using.

This can make reading the history of the repository slightly confusing.  Whether you're looking at the log
```
commit c13dc9d768ed1708dea41e7e70952ea835328d63 (origin/main)
Author: Bob Taylor <bob@example.com>
Date:   Tue Aug 8 23:18:28 2023 +1000

    Refactor the changes

commit 487e282a9483d8c08051147eb8331d5c16c4237b
Author: Bobby J Taylor <bob@example.com>
Date:   Tue Aug 8 23:18:28 2023 +1000

    Some changes

commit 96b50f824765bcb5ef936bf602cbd8c270ad245e
Author: Bobby Taylor <bobby@example.com>
Date:   Tue Aug 8 23:18:28 2023 +1000

    Initial checkin
```
or a blame view:

Thankfully `git` provides a way to coallesce multiple aliases for the same person: [`gitmailmap`](https://git-scm.com/docs/gitmailmap).  All you need to do is provide a `.mailmap` file at the root of the repository.  Description of file format.  So if Bob wanted to be called ... then ... the resulting log looks like this.
