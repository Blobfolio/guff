/*!
# Guff: Errors
*/

#[cfg(feature = "bin")]
use argyle::ArgyleError;

use parcel_css::error::{
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

	/// # CSS Parse Error.
	Css(String),

	/// # No Source.
	NoSource,

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
			Self::Browser(s) | Self::Css(s) | Self::Scss(s) => write!(f, "{} {}", self.as_str(), s),
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

macro_rules! from_parcel {
	($($ty:ty),+) => ($(
		impl From<parcel_css::error::Error<$ty>> for GuffError {
			#[inline]
			fn from(err: parcel_css::error::Error<$ty>) -> Self {
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
			Self::Css(_) => "Unable to parse CSS:",
			Self::NoSource => "An SCSS/CSS source is required.",
			Self::Scss(_) => "Unable to parse SCSS:",
			Self::SourceFileName => "File paths must be valid UTF-8.",
			Self::SourceInvalid => "Invalid/unreadable SCSS/CSS source.",

			#[cfg(feature = "bin")]
			Self::Write => "The output could not be saved to disk.",
		}
	}
}
