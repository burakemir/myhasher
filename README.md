# myhasher

Toy program that produces a sha256 hash. Isn't that something.

## Building with Bazel

See [bazelisk](https://github.com/bazelbuild/bazelisk) and [rules_rust](http://bazelbuild.github.io/rules_rust/) and [crate_universe (direct packages)](http://bazelbuild.github.io/rules_rust/crate_universe.html#direct-packages) docs.

The first time you use bazel, create and sync the `Cargo.Bazel.lock` file like so:
```
$ touch Cargo.Bazel.lock
$ CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
```

You also need to do this after `bazel clean` or any change to dependencies.
With the Bazel lockfile in sync, we can build like this:

```
$ bazel build //:myhasher
$ bazel test //:myhasher_clippy
$ bazel run //:myhasher -- $PWD/foo.txt
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

