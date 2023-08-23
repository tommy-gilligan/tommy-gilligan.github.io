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

### Eliminate Unintended Aliases

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

### Make Conflict Resolution Less Error Prone

Working with branches can result in conflicts.
It's so important to be careful when resolving conflicts because you can end up with a result that had nothing to do with actual history of either branch.

merge.tool
reasoning about conflicts can be tricky
vimdiff
[GNOME / meld · GitLab](https://gitlab.gnome.org/GNOME/meld)
[meld](https://meldmerge.org/)

### Minimize Resolving Conflicts Manually
because resolving conflicts manually is especially prone to human error, it is good to minimize how much we do it
the question with rerere is how much do you trust your previous resolution. how much does the previous resolution make sense for the current situation.
being forced to pause and consider is probably a good thing

### Finding Things Easier

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

### Stay Abrest of Changes with Autofetch

ssh agent is important especially for automation (can't be prompting for password whenever)
iterating over repositories to fetch should reveal which remotes no longer exist
automatic fetching is good to minimmize size of periodic rebase/merge but that means rebase/merging (or just testing) after fetch

VSCode apparently prompts the user for autofetch
We want to be careful not to annoy the service with too many requests
Can be an easy habbit to start many separate instances of Neovim

"git.autofetch": false,
"git.autofetchPeriod": 180,

```bash
#!/bin/bash
GIT_DIR=$(git rev-parse --show-toplevel)
cd $GIT_DIR

PID_OTHER_INSTANCES=$(pgrep -A -f $0 2>/dev/null)
for pwd_other_instance in $(pwdx $PID_OTHER_INSTANCES 2>/dev/null | cut -f2 -d' ')
do if [ "$pwd_other_instance" = "$GIT_DIR" ]
   then exit 0
   fi
done
git fetch >/dev/null
sleep 180
```

```lua
local timer = vim.loop.new_timer()
timer:start(0, 200000, vim.schedule_wrap(function()
  coroutine.create(function()
    os.execute("git-fetch")
  end)
end))
```

```bash
for i in $(find ~ -maxdepth 3 -type d -name .git | grep -Ev 'rust-master|neovim|postgres')
do cd $(echo $i | rev | cut -b5- | rev)
  git remote update
  git fetch -a
done
```

### Force Pushing Safely

When you pull from the remote, there's no telling what you might be receiving.
There is the potential for data loss when you pull a force push.  Especially
because it can easily go unnoticed.  The message you get when this happens will
look something like this: 

```
From https://github.com/tomgilligan/tomgilligan.github.io
 + 23915be...2001dbe main       -> origin/main  (forced update)
```

This relatively important message can flit by all too easily.  To give yourself
a better chance of reviewing force pushes, it can be helpful to have set 
`receive.denyNonFastForwards` (which will reject force pushes at every pull).
I wouldn't recommmend this though: often force pushes are used to delete
secrets that have been accidentally distributed.  Here there is the potential
to deny a force push of an important intended effect.

On the other side of the equation is the person who force pushed.

safer versions of force
--force-if-includes
and
--force-with-lease
github's force protections don't know about these new options?
github rules

### Hash Function Transition

[Hash Function Transition](https://git-scm.com/docs/hash-function-transition/)
[Whatever happened to SHA-256 support in Git?](https://lwn.net/Articles/898522/)

```toml
[core]
repositoryFormatVersion = 1
[extensions]
objectFormat = sha256
compatObjectFormat = sha1
```

useful for beginners
help.format = "web"

"git.autorefresh": true,
"git.fetchOnPull": false,
"git.postCommitCommand": "none",
"git.pullBeforeCheckout": false,
"git.similarityThreshold": 50,

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
