/*!
# Guff: Build
*/

use argyle::KeyWordsBuilder;
use std::path::PathBuf;



/// # Set Up CLI Arguments.
pub fn main() {
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

	let mut builder = KeyWordsBuilder::default();
	builder.push_keys([
		"-e", "--expanded",
		"-h", "--help",
		"-V", "--version",
	]);
	builder.push_keys_with_values([
		"-b", "--browsers",
		"-i", "--input",
		"-o", "--output",
	]);
	builder.save(out_path("argyle.rs"));
}

/// # Output Path.
///
/// Append the sub-path to OUT_DIR and return it.
fn out_path(stub: &str) -> PathBuf {
	std::fs::canonicalize(std::env::var("OUT_DIR").expect("Missing OUT_DIR."))
		.expect("Missing OUT_DIR.")
		.join(stub)
}
