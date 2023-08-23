+++
title = "Finding Things Easier"
description = "Some nice git config"
+++

You probably already have a tool that you use for searching your repository.
Maybe you're using whatever is built into your editor (because it's convenient)
or you're using some other tool like `ag` or `ripgrep` (for performance reasons).

`git` comes with its own search tool, `git grep`.  Don't let the name fool you.
This isn't any old `grep`.  This is fairly self-evident if you ever try to use
plain old `grep`[^oldgrep] to search your repository. Plain old `grep` is far slower.
This is because `git grep` searches files across multiple threads whereas plain
old `grep` does not.

Different flavours of regular expression can be used with `git grep`:
- basic
- extended
- perl

`git grep` can be configured to default to any one of these flavours.
It's tempting to set a default of 'extended' or 'perl' if either of these is
what is used by your text editor.

Neither of these flavours are really what is used by `vim` though.  The regular
expressions used by `vim` *resemble* 'basic' or 'extended' but they are
something else entirely.  It doesn't make sense to try to get `git grep` to
mimic `vim` regular expressions by default.

strong confidence in `git grep` working the same between systems.
BSD grep, GNU grep by configuring git grep you lose its main strength: its
ubiquity.  knowing you can use whatever 

[^oldgrep]: When I'm talking about plain old grep, I'm talking about `[Apple's BSD grep](https://opensource.apple.com/source/text_cmds/text_cmds-101.40.1/grep/)` or `[GNU grep](https://ftp.gnu.org/gnu/grep/)` or `[FreeBSD grep](https://cgit.freebsd.org/src/tree/usr.bin/grep/)`.  The `grep` that came with your operating system is probably one of these (or one of its distant relatives).

Regular expressions (REs), as defined in IEEE Std 1003.1-2004 (“POSIX.1”), come in two forms: basic regular expressions (BREs) and extended regular expressions (EREs).
man re_format 
BUGS
Having two kinds of REs is a botch.
