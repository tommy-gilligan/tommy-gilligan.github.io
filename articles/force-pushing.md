+++
title = "Force Pushing Safely"
description = "Some nice git config"
+++

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

