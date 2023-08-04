+++
title = "Zsh Keybinding Reminder"
author = "Tommy Gilligan"
description = "A small masochism to use Zsh more efficiently"
published_at = 2023-08-01T14:57:00Z
+++
I love using Neovim but at the shell I still prefer to use Emacs style
keybindings. I've never been able to remember them all, so lets build a script
to remind us of them.

First lets get a list of all the Emacs mode keybindings currently in use by zsh.

explain key combination
feedback to markdown crate (html/mdast)
phone
mention how zshzle is at the core of this
emacs style keybindings are the defaault

```zsh
% bindkey -M emacs
```

<details>
<summary>Output</summary>
<pre>
<code>
"^@" set-mark-command
"^A" beginning-of-line
"^B" backward-char
"^D" delete-char-or-list
"^E" end-of-line
"^F" forward-char
"^G" send-break
"^H" backward-delete-char
"^I" expand-or-complete
"^J" accept-line
"^K" kill-line
"^L" clear-screen
"^M" accept-line
"^N" down-line-or-history
"^O" accept-line-and-down-history
"^P" up-line-or-history
"^Q" push-line
"^R" history-incremental-search-backward
"^S" history-incremental-search-forward
"^T" transpose-chars
"^U" kill-whole-line
"^V" quoted-insert
"^W" backward-kill-word
"^X^B" vi-match-bracket
"^X^F" vi-find-next-char
"^X^J" vi-join
"^X^K" kill-buffer
"^X^N" infer-next-history
"^X^O" overwrite-mode
"^X^U" undo
"^X^V" vi-cmd-mode
"^X^X" exchange-point-and-mark
"^X*" expand-word
"^X=" what-cursor-position
"^XG" list-expand
"^Xg" list-expand
"^Xr" history-incremental-search-backward
"^Xs" history-incremental-search-forward
"^Xu" undo
"^Y" yank
"^[^D" list-choices
"^[^G" send-break
"^[^H" backward-kill-word
"^[^I" self-insert-unmeta
"^[^J" self-insert-unmeta
"^[^L" clear-screen
"^[^M" self-insert-unmeta
"^[^_" copy-prev-word
"^[ " expand-history
"^[!" expand-history
"^[\"" quote-region
"^[\$" spell-word
"^['" quote-line
"^[-" neg-argument
"^[." insert-last-word
"^[0" digit-argument
"^[1" digit-argument
"^[2" digit-argument
"^[3" digit-argument
"^[4" digit-argument
"^[5" digit-argument
"^[6" digit-argument
"^[7" digit-argument
"^[8" digit-argument
"^[9" digit-argument
"^[&lt;" beginning-of-buffer-or-history
"^[&gt;" end-of-buffer-or-history
"^[?" which-command
"^[A" accept-and-hold
"^[B" backward-word
"^[C" capitalize-word
"^[D" kill-word
"^[F" forward-word
"^[G" get-line
"^[H" run-help
"^[L" down-case-word
"^[N" history-search-forward
"^[OA" up-line-or-search
"^[OB" down-line-or-search
"^[OC" forward-char
"^[OD" backward-char
"^[OF" end-of-line
"^[OH" beginning-of-line
"^[P" history-search-backward
"^[Q" push-line
"^[S" spell-word
"^[T" transpose-words
"^[U" up-case-word
"^[W" copy-region-as-kill
"^[[200~" bracketed-paste
"^[[3~" delete-char
"^[[A" up-line-or-history
"^[[B" down-line-or-history
"^[[C" forward-char
"^[[D" backward-char
"^[_" insert-last-word
"^[a" accept-and-hold
"^[b" backward-word
"^[c" capitalize-word
"^[d" kill-word
"^[f" forward-word
"^[g" get-line
"^[h" run-help
"^[l" down-case-word
"^[n" history-search-forward
"^[p" history-search-backward
"^[q" push-line
"^[s" spell-word
"^[t" transpose-words
"^[u" up-case-word
"^[w" copy-region-as-kill
"^[x" execute-named-cmd
"^[y" yank-pop
"^[z" execute-last-named-cmd
"^[|" vi-goto-column
"^[^?" backward-kill-word
"^_" undo
" "-"~" self-insert
"^?" backward-delete-char
"\M-^@"-"\M-^?" self-insert
</code>
</pre>
</details>

```zsh
% bindkey -M emacs | shuf -n1
"^X^N" infer-next-history
```

I have no idea what `infer-next-history` means but a definition can be found at
`zshzle(1)`
```zsh
% man 1 zshzle
```

Scrolling down, we get 
```zsh
infer-next-history (^X^N) (unbound) (unbound)
  Search in the history list for a line matching the current one and fetch
  the event following it.
```

It would be nice for our script to display this next to the keybinding.
Grepping the man page for `infer-next-history` does not work though.
```zsh
% man 1 zshzle | grep -C5 'infer-next-history'
%
```

Grepping for the definition body does work though
```zsh
% man 1 zshzle | grep -C5 'Search in the history list'
```

What is going on? Why can't `grep` find this?  Let's inspect the output from
`man` with `xxd`.
```zsh
% man 1 zshzle | grep -C5 'Search in the history list' | xxd
…
00000100: 2020 2020 2020 2069 0869 6e08 6e66 0866         i.in.nf.f
00000110: 6508 6572 0872 2d08 2d6e 086e 6508 6578  e.er.r-.-n.ne.ex
00000120: 0878 7408 742d 082d 6808 6869 0869 7308  .xt.t-.-h.hi.is.
00000130: 7374 0874 6f08 6f72 0872 7908 7920 285e  st.to.or.ry.y (^
00000140: 085e 5808 585e 085e 4e08 4e29 2028 756e  .^X.X^.^N.N) (un
00000150: 626f 756e 6429 2028 756e 626f 756e 6429  bound) (unbound)
00000160: 0a20 2020 2020 2020 2020 2020 2020 2053  .              S
…
```

Here we see that we have a characte n (`0x6e`) then `0x08` and the same
character n (`0x6e`) again. What is `0x08`?

```zsh
% man ascii
…
The hexadecimal set:

00 nul   01 soh   02 stx   03 etx   04 eot   05 enq   06 ack   07 bel
08 bs    09 ht    0a nl    0b vt    0c np    0d cr    0e so    0f si
…
```

It is the backspace character.  We can strip out this pattern with:

```zsh
% man -P cat 1 zshzle | sed 's/.\x08//g'
```

That is: where there is a character followed by the backspace character,
substitute both characters with nothing (delete them).

```zsh
% man -P cat 1 zshzle | sed 's/.\x08//g' | grep -A2 "^[[:space:]]*infer-next-history"
infer-next-history (^X^N) (unbound) (unbound)
      Search in the history list for a line matching the current one
      and fetch the event following it.
```

So putting it all together now:


```zsh
# Select a random keybinding
keybinding=$(bindkey -M emacs | shuf -n1)
# Get keyboard shortcut part of the keybinding, deleting the quotes
shortcut=${${${(s. .)keybinding}[1]}[2,-2]}
bound=${${(s. .)keybinding}[2]}
man -P cat 1 zshzle | sed -n "s/.\x08//g; /^ *$bound /,/^       [^[:space:]]/p"
```


What about key bindings that I've already committed to memory?  Bit of a wasted
opportunity to tell me about a key binding like that.  A deny-list can be kept
in the `.zshrc` embedded in our script that prints keybinding tips.  All we
need to do to filter out a keybinding by name is 
```zsh
| grep -Ev ' (beginning-of-line|end-of-line)$' |
```

By default `man` will use the `less` program to make the output from `man`
scrollable i.e. 'paged'. Incidentally, you can search for `infer-next-history`
inside of less (use `/` to start entering a search term). This implication here
is that `less` strips control sequences when searching but that `grep` does
not. Where does the backspace character come from? Why is it there? Point out
source (but we're not going to use it to avoid having to fetch anything)