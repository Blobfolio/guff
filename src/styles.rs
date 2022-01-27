/*!
# Guff: Styles!
*/

use crate::GuffError;
use grass::{
	Options,
	OutputStyle,
};
use parcel_css::stylesheet::{
	MinifyOptions,
	ParserOptions,
	PrinterOptions,
	StyleSheet
};
use std::{
	ffi::OsStr,
	os::unix::ffi::OsStrExt,
	path::PathBuf,
};



/// # Parse and Process!
///
/// This will process SCSS (if applicable) and return the minified CSS output.
pub(super) fn parse(src: &[u8]) -> Result<String, GuffError> {
	// Make the path sane.
	let path: PathBuf = std::fs::canonicalize(OsStr::from_bytes(src))
		.ok()
		.filter(|x| x.is_file())
		.ok_or(GuffError::SourceInvalid)?;

	// Build or read the CSS.
	let css: String = match StyleKind::try_from(src)? {
		StyleKind::Scss => {
			let opts = Options::default()
				.style(OutputStyle::Expanded)
				.quiet(true);

			grass::from_path(
				path.to_str().ok_or(GuffError::SourceInvalid)?,
				&opts
			)
				.map_err(GuffError::from)?
		},
		StyleKind::Css => {
			std::fs::read_to_string(&path).map_err(|_| GuffError::SourceInvalid)?
		},
	};

	// Turn it over to Parcel for minification!
	let mut stylesheet = StyleSheet::parse(
		path.to_str().ok_or(GuffError::SourceInvalid)?.to_string(),
		&css,
		ParserOptions {
			nesting: true,
			css_modules: false,
			custom_media: false,
		},
	)
		.map_err(GuffError::from)?;

	stylesheet.minify(MinifyOptions::default())?;

	stylesheet.to_css(PrinterOptions {
		minify: true,
		source_map: false,
		..PrinterOptions::default()
	})
		.map(|x| x.code)
		.map_err(GuffError::from)
}



#[derive(Debug, Clone, Copy)]
/// # Style Kind.
enum StyleKind {
	Css,
	Scss,
}

impl TryFrom<&[u8]> for StyleKind {
	type Error = GuffError;

	/// # From Path Name (as bytes).
	///
	/// This just teases out the type based on the extension. We already have
	/// raw bytes, so might as well take advantage of the easy comparisons.
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		let len: usize = src.len();

		// The last two letters should always be ss.
		if len > 5 && src[len - 2..].eq_ignore_ascii_case(b"ss") {
			// A four-letter extension could be Sass.
			if src[len - 5] == b'.' {
				if src[len - 4].eq_ignore_ascii_case(&b's') {
					let mid = src[len - 3].to_ascii_lowercase();
					if mid == b'a' || mid == b'c' {
						return Ok(Self::Scss);
					}
				}
			}
			// A three-letter extension could be CSS.
			else if src[len - 4] == b'.' && src[len - 3].eq_ignore_ascii_case(&b'c') {
				return Ok(Self::Css);
			}
		}

		Err(GuffError::SourceInvalid)
	}
}
