+++
title = "Conflict resolution"
description = "Some nice git config"
+++

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
