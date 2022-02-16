/*!
# Guff: Errors
*/

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
pub(super) enum GuffError {
	/// # Argyle passthrough.
	Argue(ArgyleError),

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

	/// # Write Error.
	Write,
}

impl Error for GuffError {}

impl fmt::Display for GuffError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Css(s) | Self::Scss(s) => write!(f, "{} {}", self.as_str(), s),
			_ => f.write_str(self.as_str()),
		}
	}
}

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
				Self::Css(err.kind.reason())
			}
		}
	)+);
}

from_parcel!(MinifyErrorKind, ParserError<'_>, PrinterErrorKind);

impl GuffError {
	/// # As Str.
	pub(super) const fn as_str(&self) -> &'static str {
		match self {
			Self::Argue(e) => e.as_str(),
			Self::Css(_) => "Unable to parse CSS:",
			Self::NoSource => "An SCSS/CSS source is required.",
			Self::Scss(_) => "Unable to parse SCSS:",
			Self::SourceFileName => "File paths must be valid UTF-8.",
			Self::SourceInvalid => "Invalid/unreadable SCSS/CSS source.",
			Self::Write => "The output could not be saved to disk.",
		}
	}
}
