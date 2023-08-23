+++
title = "Git Config"
description = "Some nice git config"
+++
genesis of article: coming across fsmonitor, re-evaluating config, mark nvim, making a tool that integrates tightly with it

[Git - git-config Documentation](https://git-scm.com/docs/git-config)
[Improve Git monorepo performance with a file system monitor](https://github.blog/2022-06-29-improve-git-monorepo-performance-with-a-file-system-monitor/)
recent release of git 2.42: sha-1 warnings decreased, force pushing more normalised (github doesn't really know about this yet)

this is just stuff i came across that i found interesting
there is easily much more you can learn about git, other details etc
revisit config from time to time
never like to override defaults too much because this can be jarring when you share your system with somebody else
it's a good way to learn more about the tool generally, all its nooks and crannies, to better understand how it works and how it is intended to be used
it's also a good way to observe change in software (new features etc)
periodic re-evaluation

### 
what options are there not there that i wish were?

There is a smattering of options across `git` configuration that can be used
for selecting different versions

By setting `core.hooksPath` you can tell `git` to look for hooks in a specific
location.  Usually this is somewhere inside of the repository and the hooks are
specific to that repository.  When you want to share the same hooks across a
bunch of repositories, it makes sense to place these hooks outside of any
repositories and set `hooksPath` accordingly.

But what if you want to share hooks across repositories and you want to use
hooks that are repository-specific at the same time?  `git` doesn't give you a
good way of doing this but it is probably for the best.  Any scheme that would
allow more than one set of hooks to be in use at a time would have to specify
some kind of interaction between these different sets. This hypothetical
feature is not worth the burden of complexity.

'smart case'
principle of least surprsie
force shallow globally

#### Inspectable Defaults

Most configuration options available through `git config` have some kind of
default described in the documentation. Unfortunately some options are not
adequately described. To make matters worse, some of these defaults are
conditional.

EXAMPLE

So even when a default is adequately described it can still be
hard to know what configuration `git` is operating under.

It can be hard to know what `git` is configured to do when falling back on
default behaviour.

When `core.fsmonitor` has been set to true by the user:
```
$ git config core.fsmonitor
true
$
```
When `core.fsmonitor` has been set to false by the user:
```
$ git config core.fsmonitor
false
$
```
When `core.fsmonitor` is unset:
```
$ git config core.fsmonitor
$
```
So when `core.fsmonitor` is unset, is `fsmonitor` being used or not?

### Conclusion

The [`git-config` Documentation](https://git-scm.com/docs/git-config) (`git
help config`) really acted as a springboard for me into all of this `git`
trivia.  There is so much I haven't even yet touched upon.  Here is a Choose
Your Own Adventure

- [`git-notes` Documentation](https://git-scm.com/docs/git-notes) `git help notes`
- [`git-gc` Documentation](https://git-scm.com/docs/git-gc) `git help gc`
- [`git-worktree` Documentation](https://git-scm.com/docs/git-worktree) `git help worktree`
- [`gitattributes` Documentation](https://git-scm.com/docs/gitattributes) `git help attributes`
- [`githooks` Documentation](https://git-scm.com/docs/githooks) `git help hooks`
- [Pro Git Book](https://git-scm.com/book/en/v2)
- [Signing Your Work](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)

```
- [Signing commits - GitHub Docs](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)
 branch.<name>.description
    Branch description, can be edited with git branch --edit-description. Branch description is automatically added in the format-patch cover letter or
    request-pull summary.
```

merge.branchdesc defaults to false .  branchdesc is not really shared between different copies

good example of extending git with a background service
https://github.com/git/git/tree/master/contrib/persistent-https
vscode adopted a sensible strategy here.  background service is only used for repo that is currently 'in use'.  minimizes demand on remote.  seeing as though microsoft owns github, it's probably ok to assume that this policy is at least ok with one of the biggest providers of a remote.

rebase --exec
copies are not all that common.  but when they occur it's not tohave support for them
git config does not give you value if default is being used.  unhelpful when documentation is sometimes missing default.
sometimes defaults are conditional so something like that would be helpful
documentation's warnings of danger seem ill-advised on pull.rebase
useForceIfIncludes is fairly benign

https://ldpreload.com/blog/ssh-control
no official documentation on if they like this or not?
can just test
how to force http2 and ip6
http.version = "HTTP/2"
http 3?
ipv6?
does git use any ofthe newer features of http?

configure ssh 
you can of course use ldpreload trick to use your own libcurl with fixed options
you can use proxies too

it is easier to have more control over ssh
so maybe it makes sense to default to no http

communication config maybe doesn't make sense to dig too much into.  especially if you can't control minimum versions and we don't know whether or not new features of http/2 are actually being used.  can be useful for certain security situation.  responding to a temporary scenario.  there can be philosophical reasons.  forcing ipv6 to push its adoption.

you can use policies to force a migration to a particular type of remote

[^oldgrep]: When I'm talking about plain old grep, I'm talking about `[Apple's BSD grep](https://opensource.apple.com/source/text_cmds/text_cmds-101.40.1/grep/)` or `[GNU grep](https://ftp.gnu.org/gnu/grep/)` or `[FreeBSD grep](https://cgit.freebsd.org/src/tree/usr.bin/grep/)`.  The `grep` that came with your operating system is probably one of these (or one of its distant relatives).

Regular expressions (REs), as defined in IEEE Std 1003.1-2004 (“POSIX.1”), come in two forms: basic regular expressions (BREs) and extended regular expressions (EREs).
man re_format 
BUGS
Having two kinds of REs is a botch.
