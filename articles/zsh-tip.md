+++
title = "Zsh Keybinding Reminders"
description = "A small flagellator to learn more efficient use of Zsh"
published = true
+++
I've never been able to remember all of the handy line editing keyboard
shortcuts that are available in Zsh.  Sure, I know that
<kbd>Ctrl</kbd>+<kbd>A</kbd> goes to the beginning of the line and
<kbd>Ctrl</kbd>+<kbd>E</kbd> goes to the end of the line but there are many
other keyboard shortcuts (*key bindings*) besides these that can be quite
useful.  I'd like to learn them so that I can use Zsh more efficiently.  Maybe
a program that annoys me with random keybinding at shell startup could help me
learn.  Let's make that.

### Finding Zsh Keybindings

First I need a list of all the key bindings active in the shell. ZLE or the zsh
line editor is the part of Zsh that is responsible for binding a particular key
combination to a line editing command.  Consulting the `man` page for ZLE we
find that `bindkey` can be used to display a set of key bindings (a *keymap*).

<details>
<summary><pre><samp>% <kbd>bindkey</kbd>
"^@" set-mark-command
"^A" beginning-of-line
"^B" backward-char
"^D" delete-char-or-list
"^E" end-of-line
"^F" forward-char
"^G" send-break</samp></pre></summary>
<pre><samp>"^H" backward-delete-char
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
"^X^R" _read_comp
"^X^U" undo
"^X^V" vi-cmd-mode
"^X^X" exchange-point-and-mark
"^X*" expand-word
"^X=" what-cursor-position
"^X?" _complete_debug
"^XC" _correct_filename
"^XG" list-expand
"^Xa" _expand_alias
"^Xc" _correct_word
"^Xd" _list_expansions
"^Xe" _expand_word
"^Xg" list-expand
"^Xh" _complete_help
"^Xm" _most_recent_file
"^Xn" _next_tags
"^Xr" history-incremental-search-backward
"^Xs" history-incremental-search-forward
"^Xt" _complete_tag
"^Xu" undo
"^X~" _bash_list-choices
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
"^[," _history-complete-newer
"^[-" neg-argument
"^[." insert-last-word
"^[/" _history-complete-older
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
"^[[1;3C" forward-word
"^[[1;3D" backward-word
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
"^[~" _bash_complete-word
"^[^?" backward-kill-word
"^_" undo
" "-"~" self-insert
"^?" backward-delete-char
"\M-^@"-"\M-^?" self-insert</samp></pre>
</details>

There's a little bit of interpretation involved in understanding this keymap:

- `^[` followed by <var>character</var> means <kbd>Alt</kbd>+<kbd><var>character</var></kbd>.<br>
  If <kbd>Alt</kbd>+<kbd>U</kbd> is pressed then `up-case-word`:<br>
  `"^[u" up-case-word`
- `^` followed by <var>character</var> means <kbd>Ctrl</kbd>+<kbd><var>character</var></kbd>.<br>
  If <kbd>Ctrl</kbd>+<kbd>A</kbd> is pressed then `beginning-of-line`:<br>
  `"^A" beginning-of-line`
- Sometimes a binding consists of multiple keys pressed in sequence.<br>
  If <kbd>Ctrl</kbd>+<kbd>X</kbd> then <kbd>Ctrl</kbd>+<kbd>F</kbd> is pressed then `vi-find-next-char`:<br>
  `"^X^F" vi-find-next-char`
- <var>character</var> not preceded by `^[` or `^` is <kbd><var>character</var></kbd> without a modifier key.<br>
  If <kbd>Ctrl</kbd>+<kbd>X</kbd> then <kbd>=</kbd> is pressed then `what-cursor-position`:<br>
  `"^X=" what-cursor-position`

### Selecting a Random Keybinding 

It is easy enough to select a random keybinding, just shuffle all the
keybindings and select the first one:

<pre><samp>% <kbd>bindkey | shuf -n1</kbd>
"^[f" forward-word
</samp></pre>

### Printing Help for ZLE Commands

It's all well and good if a key is mapped to a ZLE command with an obvious name
like `forward-word` but what if that's not the case.  Selecting another random
keybinding:

<pre><samp>% <kbd>bindkey | shuf -n1</kbd>
"^@" set-mark-command
</samp></pre>

What in the world is `set-mark-command`?  The `man` page has the answer!

<pre><samp>% <kbd>man 1 zshzle</kbd></samp></pre>

After scrolling a bit:

<pre><samp>set-mark-command (^@) (unbound) (unbound)
      Set the mark at the cursor position.  If called with a negative
      numeric argument, do not set the mark but deactivate the region
      so that it is no longer highlighted (it is still usable for
      other purposes).  Otherwise the region is marked as active.
</samp></pre>

So it should be possible to just grep for this documentation right?
<pre><samp>% <kbd>man 1 zshzle | grep -A1 set-mark-command</kbd>
% </samp></pre>

Curiously, `grep` doesn't return any matches.  What is going on here?  Inspecting the output from `man` with `xxd`
<pre><samp>% <kbd>man 1 zshzle | xxd | cut -b11-</kbd>
&hellip;
0a20 2020 2020 2020 7308 7365 0865 7408  .       s.se.et.
742d 082d 6d08 6d61 0861 7208 726b 086b  t-.-m.ma.ar.rk.k
2d08 2d63 0863 6f08 6f6d 086d 6d08 6d61  -.-c.co.om.mm.ma
0861 6e08 6e64 0864 2028 5e08 5e40 0840  .an.nd.d (^.^@.@
2920 2875 6e62 6f75 6e64 2920 2875 6e62  ) (unbound) (unb
6f75 6e64 290a 2020 2020 2020 2020 2020  ound).
&hellip;</samp></pre>

The label for `set-mark-command` is present but it is somewhat mangled. Every
letter in the label is followed by `0x08` and then a repitition of the original
letter.  Presumably this strange sequence of characters is to format things
nicely for display.  This output is a bit annoying to have to deal with but
it's not hard to do so by filtering through `sed`:

<pre><samp>% <kbd>man 1 zshzle | sed 's/.\x08//g</kbd></samp></pre>

This `sed` filter is essentailly "when there is a character followed by the
`0x08` character, substitute both characters with nothing (delete them)." By
chaining `grep` on the end, it is now possible to search the documentation for
a ZLE command by name:

<pre><samp>% <kbd>man 1 zshzle | sed -E 's/^ *|\x08.//g' | grep -A1 set-mark-command</kbd>
set-mark-command in Emacs mode, or by visual-mode in Vi mode) is
enabled by default; consult this reference for more information.
--
set-mark-command (^@) (unbound) (unbound)
Set the mark at the cursor position.  If called with a negative
--
set-mark-command or exchange-point-and-mark.  Note that whether
or not the region is active has no effect on its use within</samp></pre>

There are a few unintended matches here.  The match in the middle is what I'm
interested in.  Tightening up the regular expression:
<pre><samp>% <kbd>man 1 zshzle | sed -E 's/^ *|\x08.//g' | grep -A1 '^set-mark-command ('</kbd>
set-mark-command (^@) (unbound) (unbound)
Set the mark at the cursor position.  If called with a negative</samp></pre>

Switching from `grep` to `sed` and generalising to create a Zsh function:

```zsh
function print_zle_command_help() {
  man 1 zshzle | sed -E -n "
  # unindent and delete special formatting characters
  s/^ *|\x08.//g
  # so that $1 can match the start of a command's entry in manpage
  # up until the next blank line
  /^$1 \(/,/^$/ {
      # delete header line and blank lines
      /^$1 \(|^$/d
      # print entry
      p
  }"
}
```

Calling `print_zle_command_help` with `set-mark-command`
<pre><samp>% <kbd>print_zle_command_help set-mark-command</kbd>
Set the mark at the cursor position.  If called with a negative
numeric argument, do not set the mark but deactivate the region
so that it is no longer highlighted (it is still usable for
other purposes).  Otherwise the region is marked as active.
</samp></pre>

### Printing Random Keybinding on Shell Startup

To print a random keybinding every time the shell starts, add the
`print_zle_command_help` function and this `print_random_keybinding` function
to `~/.zshrc`.
```zsh
function print_random_keybinding() {
    # Select a random keybinding
    local keybinding=$(bindkey | shuf -n1)
    # Get keyboard shortcut part of the keybinding, deleting the quotes
    local shortcut=${${${(s. .)keybinding}[1]}[2,-2]}
    # Get the command part of the keybinding
    local command_name=${${(s. .)keybinding}[2]}
    echo $shortcut
    print_zle_command_help $command_name
}
```

Combining selecting a random keybinding and printing it:

<script src="https://gist.github.com/tommy-gilligan/6c9b4de4def9702c80364fba43f6f938.js"></script>
