# Contributor guidelines

## Minimum required stable Rust version

This project is guaranteed to build with the latest stable Rust toolchain. More specifically, the toolchain that is installed by default on GitHub's `ubuntu-latest` runner. You can see [here](https://github.com/actions/virtual-environments/blob/main/images/linux/Ubuntu2004-Readme.md#rust-tools) what version that currently is.

Note that the toolchain required to build this project is distinct from the toolchain required to build the rustdoc JSON that this project relies on. See below for more info.

## Minimum required nightly Rust version

Since the rustdoc JSON format still changes in incompatible ways, there is a lower bound on what nightly version you can use. For regular users, that minimal nightly version is mentioned in the README.md. For developers however, a more recent version can be needed. This is because even though the rustdoc JSON format is unchanged, its output can change. See [this PR](https://github.com/Enselic/cargo-public-api/pull/84) for just one example.

Our CI runs every night, so any problems are generally detected quickly. If `cargo test` fails, make sure you have a recent enough nightly toolchain installed.

# Tips to work on this tool

## Running CI steps locally

Run `./scripts/run-ci-locally.sh`.

## Run local copy of `cargo-public-api` on an arbitrary crate

There are two ways. You can either do:
```
% cd ~/src/arbitrary-crate
% cargo run --manifest-path ~/src/cargo-public-api/cargo-public-api/Cargo.toml
```
or you can do
```
% cd ~/src/cargo-public-api
% cargo run --bin cargo-public-api -- --manifest-path ~/src/arbitrary-crate/Cargo.toml
```
In the first case `--manifest-path` is interpreted by `cargo` itself, and in the second case `--manifest-path` is interpreted by `cargo-public-api`.

You can also combine both ways:
```
% cd /does/not/matter
% cargo run --manifest-path ~/src/cargo-public-api/cargo-public-api/Cargo.toml -- --manifest-path ~/src/arbitrary-crate/Cargo.toml
```

## Use custom rustdoc JSON toolchain

If you have built rustdoc yourself to try some rustdoc JSON fix, you can run `cargo public-api` with your [custom toolchain](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#creating-a-rustup-toolchain) like this:

```
cargo public-api --rustdoc-json-toolchain +custom
```

Another option is the `RUSTDOC_JSON_OVERRIDDEN_TOOLCHAIN_HACK` env var. Use it like this:
```
RUSTDOC_JSON_OVERRIDDEN_TOOLCHAIN_HACK=+custom ./scripts/run-ci-locally.sh
```

## Code coverage

Exploring code coverage is a good way to ensure we have broad enough tests. This is the command I use personally to get started:

```bash
cd public-api
cargo llvm-cov --html && open ../target/llvm-cov/html/index.html
```

Which requires you to have done `cargo install cargo-llvm-cov` first.

# Maintainer guidelines

Please see [MAINTAINER.md](./MAINTAINER.md).
