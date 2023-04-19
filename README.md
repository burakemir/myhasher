# myhasher

Toy program that produces a sha256 hash. Isn't that something.

## Building with cargo

```
$ cargo build
$ cargo run -- foo.txt

d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5 foo.txt
```

## Building with Bazel

See [bazelisk](https://github.com/bazelbuild/bazelisk) and [rules_rust](http://bazelbuild.github.io/rules_rust/) and [crate_universe](http://bazelbuild.github.io/rules_rust/crate_universe.html) docs.

The first time you use bazel, create and sync the `Cargo.Bazel.lock` file like so:
```
$ touch Cargo.Bazel.lock
$ CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
```

You also need to do this when the file is out of sync with `Cargo.toml` (when you change dependencies there).
With the Bazel lockfile in sync, we can build like this:

```
$ bazelisk build //:myhasher
$ bazelisk run //:myhasher -- $PWD/foo.txt
INFO: Analyzed target //:myhasher (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //:myhasher up-to-date:
  bazel-bin/myhasher
INFO: Elapsed time: 0.132s, Critical Path: 0.00s
INFO: 1 process: 1 internal.
INFO: Build completed successfully, 1 total action
INFO: Running command line: bazel-bin/myhasher /home/buraq/projects/rust/myhasher/foo.txt
d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5 /home/buraq/projects/rust/myhasher/foo.txt
```

This may seem pointless since there is no BUILD target that
does not gets its dependencies from `Cargo.toml`. This is
still a toy program, but it can change to include targets
with "direct dependencies" which will only be known to Bazel.
