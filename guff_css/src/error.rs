/*!
# Guff: Errors
*/

#[cfg(feature = "bin")]
use argyle::ArgyleError;

use lightningcss::error::{
	MinifyErrorKind,
	ParserError,
	PrinterErrorKind,
};
use std::{
	error::Error,
	fmt,
};



#[derive(Debug, Clone)]
/// # Error type.
pub enum GuffError {
	#[cfg(feature = "bin")]
	/// # Argyle passthrough.
	Argue(ArgyleError),

	/// # Browser.
	Browser(String),

	#[cfg(feature = "bin")]
	/// # Invalid CLI.
	Cli(String),

	/// # CSS Parse Error.
	Css(String),

	#[cfg(feature = "bin")]
	/// # No Source.
	NoSource,

	/// # Invalid Path Extension.
	PathExt,

	/// # Invalid Path UTF-8.
	PathUtf8,

	/// # SCSS Parse Error.
	Scss(String),

	/// # Source File Name.
	SourceFileName,

	/// # Invalid Source.
	SourceInvalid,

	#[cfg(feature = "bin")]
	/// # Write Error.
	Write,
}

impl Error for GuffError {}

impl fmt::Display for GuffError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Browser(s) | Self::Css(s) | Self::Scss(s) => write!(f, "{} {s}", self.as_str()),

			#[cfg(feature = "bin")]
			Self::Cli(s) => write!(f, "{} {s}", self.as_str()),

			_ => f.write_str(self.as_str()),
		}
	}
}

#[cfg(feature = "bin")]
impl From<ArgyleError> for GuffError {
	#[inline]
	fn from(err: ArgyleError) -> Self { Self::Argue(err) }
}

impl From<Box<grass::Error>> for GuffError {
	#[inline]
	fn from(err: Box<grass::Error>) -> Self { Self::Scss(err.to_string()) }
}

/// # Helper: From Parcel.
macro_rules! from_parcel {
	($($ty:ty),+) => ($(
		impl From<lightningcss::error::Error<$ty>> for GuffError {
			#[inline]
			fn from(err: lightningcss::error::Error<$ty>) -> Self {
				Self::Css(err.kind.to_string())
			}
		}
	)+);
}

from_parcel!(MinifyErrorKind, ParserError<'_>, PrinterErrorKind);

impl GuffError {
	#[must_use]
	/// # As Str.
	pub const fn as_str(&self) -> &'static str {
		match self {
			#[cfg(feature = "bin")]
			Self::Argue(e) => e.as_str(),

			Self::Browser(_) => "Invalid browser:",

			#[cfg(feature = "bin")]
			Self::Cli(_) => "Invalid/unknown option:",

			Self::Css(_) => "Unable to parse CSS:",

			#[cfg(feature = "bin")]
			Self::NoSource => "An SCSS/CSS source is required.",

			Self::PathExt => "Paths must contain a .css, .sass, or .scss extension.",
			Self::PathUtf8 => "Paths must be valid UTF-8.",
			Self::Scss(_) => "Unable to parse SCSS:",
			Self::SourceFileName => "File paths must be valid UTF-8.",
			Self::SourceInvalid => "Invalid/unreadable SCSS/CSS source.",

			#[cfg(feature = "bin")]
			Self::Write => "The output could not be saved to disk.",
		}
	}
}
