+++
title = "Autofetch"
description = "Some nice git config"
+++

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
