+++
title = "Conflict resolution"
description = "Some nice git config"
published = false
+++

Source 
: A DRY-er YAML file that makes use of aliases.

Target 
: A flattened version of *Source* that does not contain any aliases and can be used by Github. 

On `pre-commit`:

1. Produce a new version of *Target* from *Source*
2. Error if there are unstaged changes to *Target*

Lets focus on step 1 first:

# Produce a new version of *Target* from *Source*

Lets make a new `flatten-yaml` command that has a usage syntax similar to the familiar `cp` and `mv` commands ie. it accepts positional arguments that refer to *Source* and *Target* files.  Running

```
% cargo run --bin flatten-yaml $SOURCE $TARGET
```

Will produce
```yaml


```

The [`clio`](https://github.com/aj-bagwell/clio) crate makes handling file-like parameters a snap.  A simple example of a program using `clio`.
```rust
// a cat replacement
fn main() -> clio::Result<()> {
    for arg in std::env::args_os() {
        let mut input = clio::Input::new(&arg)?;
        std::io::copy(&mut input, &mut std::io::stdout())?;
    }
    Ok(())
}
```

This only needs slight alteration to accept a single input argument and a single output argument:
```rust
fn main() -> clio::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let input = clio::Input::new(&args.next().expect("Input argument absent"))?;
    let output = clio::Output::new(&args.next().expect("Output argument absent"))?;
    flatten_yaml(input, output).unwrap();

    Ok(())
}
```

Now for the meat of the program, the YAML flattening.  Using the `serde_yaml` crate, read from `input`
```rust
fn flatten_yaml(input: clio::Input, output: clio::Output) -> serde_yaml::Result<()> {
    let flattened: serde_yaml::Value = serde_yaml::from_reader(input)?;
    serde_yaml::to_writer(output, &flattened)
}
```

As far as creating a program to flatten YAML is concerned, that's all there is to it. Now for the second step of erroring if there are any unstaged changes to *Target*.

# Error if there are unstaged changes to *Target*

The [`devx`](https://github.com/elastio/devx) crate provides a handy entry point for writing a `git` `pre-commit` hook in Rust.  From the [documentation](https://docs.rs/devx-pre-commit/latest/devx_pre_commit/):

```rust
use devx_pre_commit::{PreCommitContext, locate_project_root};
use anyhow::Result;
use std::{ffi::OsStr, path::PathBuf};

fn run_hook() -> Result<()> {
    let mut ctx = PreCommitContext::from_git_diff(locate_project_root()?)?;

    // Optionally filter out the files you don't want to format
    ctx.retain_staged_files(|path| {
        path.components().all(|it| it.as_os_str() != OsStr::new("generated"))
    });

    // Run `cargo fmt` against the crates with staged rust source files
    ctx.rustfmt()?;

    // Stage all the changes potenitally introduced by rustfmt
    // It is super-important to call this method at the end of the hook
    ctx.stage_new_changes()?;
    Ok(())
}

fn main() -> Result<()> {
    if let Some(true) = std::env::args().next().map(|it| it.contains("pre-commit")) {
        return run_hook();
    }
    match std::env::args().nth(1).expect("No args").as_str() {
        "install-pre-commit-hook" => {
            devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;
        }
        _ => {
            eprintln!("Hi, this is a dev cli, here are the available commands...");
        }
    }
    Ok(())
}
```

`main()` seems to be exactly what we want.  The documentation example's `run_hook()` however is concerned with code formatting, which we aren't interested in here.  First lets adapt the filter so that the hook only cares about YAML flattening:
```rust
// Optionally filter out the files you don't want to flatten
let source = "ci.yaml"
let source_path_components = Path::new(&source).components();
ctx.retain_staged_files(|path| path == source_path_components);
```

Looking at the source of `rustfmt()`

```rust
cmd!(std::env::var("CARGO")
    .as_ref()
    .map(Deref::deref)
    .unwrap_or("cargo"))
.arg("fmt")
.arg("--package")
.args(touched_crates)
.run()?;

Ok(())
```

Adapting this to run `flattenyaml` with:
- a source `ci.yaml` at the root of the repository
- a target `.github/workflows/ci.yaml`

```rust
let target = ".github/workflows/ci.yaml";
cmd!(std::env::var("CARGO")
    .as_ref()
    .map(Deref::deref)
    .unwrap_or("cargo"))
.arg("xtask")
.arg("flattenyaml")
.arg(&source)
.arg(&target)
.run()?;

Ok(())
```

We end up with a `run_hook()` that looks like
```rust
use devx_pre_commit::{PreCommitContext, locate_project_root};
use anyhow::Result;
use std::{ffi::OsStr, path::PathBuf};

fn run_hook() -> Result<()> {
    let mut ctx = PreCommitContext::from_git_diff(locate_project_root()?)?;

    // Optionally filter out the files you don't want to flatten
    let source = "ci.yaml"
    let source_path_components = Path::new(&source).components();
    ctx.retain_staged_files(|path| path == source_path_components);

    // Run `cargo fmt` against the crates with staged rust source files
    let target = ".github/workflows/ci.yaml";
    cmd!(std::env::var("CARGO")
        .as_ref()
        .map(Deref::deref)
        .unwrap_or("cargo"))
    .arg("xtask")
    .arg("flattenyaml")
    .arg(&source)
    .arg(&target)
    .run()?;

    // Stage all the changes potenitally introduced by rustfmt
    // It is super-important to call this method at the end of the hook
    ctx.stage_new_changes()?;
    Ok(())
}

fn main() -> Result<()> {
    if let Some(true) = std::env::args().next().map(|it| it.contains("pre-commit")) {
        return run_hook();
    }
    match std::env::args().nth(1).expect("No args").as_str() {
        "install-pre-commit-hook" => {
            devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;
        }
        _ => {
            eprintln!("Hi, this is a dev cli, here are the available commands...");
        }
    }
    Ok(())
}
```

The example's `run_hook()` stages new changes produced by the hook.  This is not the behaviour I want for this hook.  Instead of staging the changes, I want the hook to error (and prevent committing) if there are changes to the flattened Target.

Adapting this for 

[`cargo-xtask`](https://github.com/matklad/cargo-xtask) is appropriate here.  In my project I already have a bunch of xtasks.
Using `clap`'s derive

Is Github being paternalistic/materialistic in pushing you to create separate repos for composing actions?  It seems that way.  Especially through noncommunication.  Is this a useful pressure for their users?  To end up creating a separate action an easier path is to cut down on duplication within your yaml through more lightweight means.
Other reason could be that: allowing aliases makes YAML accepted by Github more error prone.  Or prone to security issues.
Maybe at the end of the day the fact that I feel like I need this mechanism maybe just means that I'm Doing It Wrong.  I'm not using a thing the way it is meant to be used but I would contend that the path to resolving such an issue is through refactoring and using aliases is a good mechanism to refactor this into something easier to grok.  From there we can work out what we should split out as actions etc.  (Direct link to 

Should pre-commit ever change files?  Should it stage files?  I'm saying no.  It should just tell you what you have forgotten to do.  It is essential that it runs very fast.
Being able to pass arbitrary list of files.  So you can use your commit hook in different contexts.

Self-install?  This silent operation is a bit annoying.  It can be hard to tell if your hooks are up to date though.
A hook needs to be able to tell from itself if it is updated.
It should be able to update itself as a way to confirm that the installer is working (but this should be optional)
It should provide instructions on how to update it that should always work from current repo.

Running the hook on CI should be able to tell you this information too (if it fails on CI, probably a user does not have up-to-date hook)
Ignore installation entirely for now.

Tried something similar to what devx was doing but didn't like it because: silent over-write, no guarantee you're up to date.
There is an assumption that the files you're checking are not interdependent.
Is somebody using a dark pattern or not? How do you test for that?
