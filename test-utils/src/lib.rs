// deny in CI, only warn here
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

use std::path::Path;
use std::path::PathBuf;

mod create_test_git_repo;
pub use create_test_git_repo::create_test_git_repo;

pub mod assert_or_bless;
pub use assert_or_bless::assert_eq_or_bless;
pub use assert_or_bless::write_to_file_atomically;

#[must_use]
pub fn rustdoc_json_path_for_crate(test_crate: &str) -> PathBuf {
    rustdoc_json_path_for_crate_impl(test_crate, None)
}

#[must_use]
pub fn rustdoc_json_path_for_crate_with_target_dir(
    test_crate: &str,
    target_dir: impl AsRef<Path>,
) -> PathBuf {
    rustdoc_json_path_for_crate_impl(test_crate, Some(target_dir.as_ref()))
}

/// Helper to get the path to a freshly built rustdoc JSON file for the given
/// test-crate.
#[must_use]
fn rustdoc_json_path_for_crate_impl(test_crate: &str, target_dir: Option<&Path>) -> PathBuf {
    let mut builder = rustdoc_json::Builder::default()
        .toolchain("nightly".to_owned())
        .manifest_path(&format!("{}/Cargo.toml", test_crate))
        // The test framework is unable to capture output from child processes (see
        // https://users.rust-lang.org/t/cargo-doesnt-capture-stderr-in-tests/67045/4),
        // so build quietly to make running tests much less noisy
        .quiet(true);

    if let Some(target_dir) = target_dir {
        builder = builder.target_dir(target_dir);
    }

    builder.build().unwrap()
}

/// Helper to get a String of freshly built rustdoc JSON for the given
/// test-crate.
#[must_use]
#[allow(dead_code)]
pub fn rustdoc_json_str_for_crate(test_crate: &str) -> String {
    std::fs::read_to_string(rustdoc_json_path_for_crate(test_crate)).unwrap()
}
