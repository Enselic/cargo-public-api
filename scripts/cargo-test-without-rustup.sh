#!/usr/bin/env bash
set -o nounset -o pipefail -o errexit -o xtrace

RUSTUP="$(which rustup)"
restore_rustup() {
    mv "${RUSTUP}-disabled" "${RUSTUP}"
}
trap restore_rustup EXIT
mv "${RUSTUP}" "${RUSTUP}-disabled"
cargo test --locked --features test-without-rustup-in-path without_rustup_in_path
