/*!
# Guff: Styles!
*/

use crate::{
	Agents,
	GuffError,
};
use grass::{
	Options,
	OutputStyle,
};
use lightningcss::{
	stylesheet::{
		MinifyOptions,
		ParserFlags,
		ParserOptions,
		PrinterOptions,
		StyleSheet,
	},
	targets::{
		Browsers,
		Features,
		Targets,
	},
};
use std::path::Path;
use trimothy::TrimMut;



#[derive(Debug)]
/// # CSS Stylesheet.
///
/// This struct is used to parse and validate CSS/SCSS content, and
/// optionally minify it for production use.
///
/// Note: CSS modules are not supported, however SCSS imports are.
///
/// ## Use
///
/// If you already have the CSS gathered into a string, you can instantiate
/// this object using `From<&str>` or `From<String>`.
///
/// More often, you'll want to load the contents directly from a file, which
/// can be done using `TryFrom<&Path>`. If the file extension is `.sass` or
/// `.scss`, the contents will be run through the SCSS interpreter to generate
/// the CSS, otherwise if the file extension is `.css`, the file contents will
/// be used directly.
///
/// Note: SCSS is only supported when read from a file.
///
/// Once initialized, you can convert your [`Css`] object into an owned string
/// (i.e. browser-ready CSS) using the [`Css::take`] or [`Css::minified`]
/// methods.
///
/// ## Examples
///
/// ```
/// use guff_css::Css;
/// use std::path::Path;
///
/// // Load, validate, and return CSS from a file.
/// let css = Css::try_from(Path::new("../skel/style.css"))
///     .unwrap()
///     .take()
///     .unwrap();
///
/// // The same thing, but starting from SCSS, and minifying!
/// let css = Css::try_from(Path::new("../skel/style.scss"))
///     .unwrap()
///     .minified(None)
///     .unwrap();
/// ```
pub struct Css<'a> {
	/// # File Path.
	path: &'a str,

	/// # CSS.
	css: String,
}

impl From<&str> for Css<'_> {
	fn from(src: &str) -> Self { Self::from(src.to_owned()) }
}

impl From<String> for Css<'_> {
	fn from(css: String) -> Self {
		Self {
			path: "stylesheet.css",
			css,
		}
	}
}

impl<'a> TryFrom<&'a Path> for Css<'a> {
	type Error = GuffError;

	/// # From File.
	///
	/// This will attempt to load the contents of the given file, using its
	/// extension as a filetype hint.
	///
	/// If the file ends with `.sass` or `.scss`, it is assumed to be SCSS, and
	/// will be run through the SCSS compiler to produce valid CSS.
	///
	/// If the file ends with `.css`, the contents are presumed to already be
	/// valid CSS.
	///
	/// ## Errors
	///
	/// If the file is missing, unreadable, or does not end with a CSS or SCSS
	/// extension, an error will be returned. For SCSS, processing errors will
	/// be bubbled up if encountered.
	fn try_from(src: &'a Path) -> Result<Self, Self::Error> {
		// The path has to be valid UTF-8.
		let path: &str = src.as_os_str().to_str().ok_or(GuffError::PathUtf8)?;

		// Come up with CSS.
		let css: String = match StyleKind::try_from(path.as_bytes())? {
			// The CSS has to be built from SASS.
			StyleKind::Scss => grass::from_path(
				path,
				&Options::default()
					.style(OutputStyle::Expanded)
					.quiet(true)
			)?,
			// The file is already CSS; we just need to read it!
			StyleKind::Css => std::fs::read_to_string(path)
				.map_err(|_| GuffError::SourceInvalid)?
		};

		Ok(Self { path, css })
	}
}

impl Css<'_> {
	/// # Clean Up.
	///
	/// This trims the string and removes any UTF-8 BOM markers.
	fn prepare(&mut self) {
		// Strip out UTF-8 BOM characters.
		self.css.retain(|c| c != '\u{feff}');

		// Trim it.
		self.css.trim_mut();
	}

	/// # Minify.
	///
	/// This works just like [`Css::take`], except the CSS is aggressively
	/// minified before being returned.
	///
	/// Speaking of aggression, the latest and greatest CSS features are
	/// sometimes leveraged for additional space savings. The `browsers`
	/// argument can be used to override this behavior, maintaining backward
	/// compatibility with the browsers specified.
	///
	/// See the documentation for [`Agents`] for more information.
	///
	/// ## Errors
	///
	/// If the CSS cannot be parsed or errors occur during minification, an
	/// error will be returned.
	pub fn minified(mut self, browsers: Option<Agents>) -> Result<String, GuffError> {
		self.prepare();
		let Self { path, css } = self;

		// Empty CSS can't be minified any further. Haha.
		if css.is_empty() { Ok(css) }
		// Minify it!
		else {
			// Parse the stylesheet as CSS.
			let mut stylesheet = StyleSheet::parse(&css, ParserOptions {
				filename: path.to_owned(),
				css_modules: None,
				source_index: 0,
				error_recovery: false,
				warnings: None,
				flags: ParserFlags::NESTING,
			})?;

			// Convert our Agents into a parcel Browsers object.
			let browsers = browsers.and_then(Option::<Browsers>::from);

			// Minify it.
			stylesheet.minify(MinifyOptions {
				targets: Targets {
					browsers,
					include: Features::MediaRangeSyntax,
					exclude: Features::empty(),
				},
				..MinifyOptions::default()
			})?;

			// Turn it back into a string.
			let out = stylesheet.to_css(PrinterOptions {
				minify: true,
				targets: Targets {
					browsers,
					include: Features::MediaRangeSyntax,
					exclude: Features::empty(),
				},
				..PrinterOptions::default()
			})?;

			// Done!
			Ok(out.code)
		}
	}

	/// # Take.
	///
	/// Validate and return the CSS as an owned string.
	///
	/// This will trim whitespace and UTF-8 BOM markers, but otherwise leave
	/// the contents as-were.
	///
	/// ## Errors
	///
	/// If the CSS cannot be parsed, an error will be returned.
	pub fn take(mut self) -> Result<String, GuffError> {
		self.prepare();
		let Self { path, css } = self;

		// Make sure it is parseable.
		if ! css.is_empty() {
			StyleSheet::parse(&css, ParserOptions {
				filename: path.to_owned(),
				css_modules: None,
				source_index: 0,
				error_recovery: false,
				warnings: None,
				flags: ParserFlags::NESTING,
			})?;
		}

		Ok(css)
	}
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Style Kind.
enum StyleKind {
	/// # CSS.
	Css,

	/// # SCSS.
	Scss,
}

impl TryFrom<&[u8]> for StyleKind {
	type Error = GuffError;

	/// # From Path Name (as bytes).
	///
	/// This just teases out the type based on the extension. We already have
	/// raw bytes, so might as well take advantage of the easy comparisons.
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		// Should end in SS either way.
		if let [src @ .., b's' | b'S', b's' | b'S'] = src {
			match src {
				[.., 0..=46 | 48..=91 | 93..=255, b'.', b'c' | b'C'] => Ok(Self::Css),
				[.., 0..=46 | 48..=91 | 93..=255, b'.', b's' | b'S', b'a' | b'c' | b'A' | b'C'] => Ok(Self::Scss),
				_ => Err(GuffError::PathExt),
			}
		}
		else { Err(GuffError::PathExt) }
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_stylekind() {
		for (file, expected) in [
			("/foo/bar.css", Some(StyleKind::Css)),
			("/foo/bar.sass", Some(StyleKind::Scss)),
			("/foo/bar.scss", Some(StyleKind::Scss)),
			("a.css", Some(StyleKind::Css)),
			("a.sass", Some(StyleKind::Scss)),
			("a.scss", Some(StyleKind::Scss)),
			(".css", None),
			(".sass", None),
			(".scss", None),
			("/foo/.css", None),
			("/foo/.sass", None),
			("/foo/.scss", None),
			("/foo/bar.jpeg", None),
		] {
			assert_eq!(StyleKind::try_from(file.as_bytes()).ok(), expected);
			let file = file.to_uppercase();
			assert_eq!(StyleKind::try_from(file.as_bytes()).ok(), expected);
		}
	}
}
