/*!
# Guff: Build
*/

use dactyl::NiceU32;
use serde::Deserialize;
use std::{
	collections::HashMap,
	fs::{
		File,
		Metadata,
	},
	io::Write,
	num::NonZeroU32,
	path::{
		Path,
		PathBuf,
	},
};



/// # Can I Use? publishes their data here.
const DATA_URL: &str = "https://github.com/Fyrd/caniuse/raw/main/fulldata-json/data-2.0.json";

/// # The Version Triple.
///
/// This holds the Parcel value, major number, and release date.
type Triple = (u32, u32, u32);



/// # Pull (supported) browser version data from Can I Use.
///
/// This ultimately compiles a static array of (Kind, Tuple) data, where each
/// tuple holds:
/// 0. The version as Parcel wants it.
/// 1. The major version number.
/// 2. The release date (UTC).
///
/// The versions are sorted descendingly, and capped at 16 entries because,
/// come on! Haha.
///
/// That's it!
pub fn main() {
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

	let raw: Raw = serde_json::from_str(&fetch()).expect("Unable to parse raw.");
	let out: String = process(raw);

	let cache = out_path("guff-browsers.rs");
	File::create(cache)
		.and_then(|mut f| f.write_all(out.as_bytes()).and_then(|_| f.flush()))
		.expect("Unable to save browser data.");
}

/// # Download/Cache Raw JSON.
fn fetch() -> String {
	// Is it cached?
	let cache = out_path("caniuse.json");
	if let Some(x) = try_cache(&cache) {
		return x;
	}

	// Download it.
	let out: String = ureq::get(DATA_URL)
		.set("user-agent", "Mozilla/5.0")
		.call()
		.and_then(|r| r.into_string().map_err(Into::into))
		.expect("Unable to download data.");

	// Try to save for next time.
	let _res = File::create(cache)
		.and_then(|mut f| f.write_all(out.as_bytes()).and_then(|_| f.flush()));

	// Return the raw value.
	out
}

/// # Process the Data.
fn process(raw: Raw) -> String {
	let all: HashMap<Agent, Vec<Triple>> = raw.agents.into_iter()
		.filter_map(|(k, mut v)| {
			let agent = Agent::try_from(k.as_str()).ok()?;
			v.version_list.sort_by(|a, b| b.era.cmp(&a.era));

			let mut releases: Vec<Triple> = v.version_list.into_iter()
				.filter_map(|v2| {
					let date = v2.release_date?;
					let (parcel, major) = parse_version(&v2.version)?;
					Some((parcel, major, date))
				})
				.collect();

			// We don't need more than 20 of these things.
			if 16 < releases.len() { releases.truncate(16); }

			Some((agent, releases))
		})
		.collect();

	let all_len: usize = all.len();
	let mut all = all.into_iter()
		.map(|(k, v)| {
			let v = v.into_iter().map(|v2| format!(
				"({}, {}, {})",
				NiceU32::with_separator(v2.0, b'_'),
				NiceU32::with_separator(v2.1, b'_'),
				NiceU32::with_separator(v2.2, b'_'),
			))
				.collect::<Vec<String>>()
				.join(", ");

			format!("(Agent::{}, &[{}])", k.as_str(), v)
		})
		.collect::<Vec<String>>();
	all.sort_unstable();

	let out = format!(
		"#[allow(clippy::type_complexity)]
/// # Browser Data.
///
/// Each tuple holds the Parcel version, major version, and release date.
const BROWSERS: [(Agent, &[(u32, u32, u32)]); {}] = [\n\t{}\n];
",
		all_len,
		all.join(",\n\t"),
	);

	out
}



#[derive(Deserialize)]
/// # Agents.
struct Raw {
	agents: HashMap<String, RawAgent>
}

#[derive(Deserialize)]
/// # Agent.
struct RawAgent {
	version_list: Vec<RawAgentVersions>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
/// # Agent Version List.
struct RawAgentVersions {
	version: String,
	release_date: Option<u32>,
	era: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Agent Kind.
enum Agent {
	Android,
	Chrome,
	Edge,
	Firefox,
	Ie,
	Ios,
	Opera,
	Safari,
	Samsung,
}

impl Agent {
	const fn as_str(self) -> &'static str  {
		match self {
			Self::Android => "Android",
			Self::Chrome => "Chrome",
			Self::Edge => "Edge",
			Self::Firefox => "Firefox",
			Self::Ie => "Ie",
			Self::Ios => "Ios",
			Self::Opera => "Opera",
			Self::Safari => "Safari",
			Self::Samsung => "Samsung",
		}
	}
}

impl TryFrom<&str> for Agent {
	type Error = ();

	fn try_from(src: &str) -> Result<Self, Self::Error> {
		match src.trim() {
			"android" => Ok(Self::Android),
			"chrome" => Ok(Self::Chrome),
			"edge" => Ok(Self::Edge),
			"firefox" => Ok(Self::Firefox),
			"ie" => Ok(Self::Ie),
			"ios_saf" => Ok(Self::Ios),
			"opera" => Ok(Self::Opera),
			"safari" => Ok(Self::Safari),
			"samsung" => Ok(Self::Samsung),
			_ => Err(()),
		}
	}
}



/// # Out path.
///
/// This generates a (file/dir) path relative to `OUT_DIR`.
fn out_path(name: &str) -> PathBuf {
	let dir = std::env::var("OUT_DIR").expect("Missing OUT_DIR.");
	let mut out = std::fs::canonicalize(dir).expect("Missing OUT_DIR.");
	out.push(name);
	out
}

/// # Parse Version.
///
/// This parses a version from a string, generating the packed `u32` Parcel
/// expects, and also returning the major for reference.
fn parse_version(src: &str) -> Option<(u32, u32)> {
	// Strip the revision and split on dots.
	let mut version = src.split('-')
		.next()?
		.split('.');

	// Major is first and required.
	let major = version.next().and_then(|v| v.parse::<u32>().ok())?;

	// Minor and patch follow, but may be zero.
	let minor = version.next().and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
	let patch = version.next().and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);

	let v: u32 = (major & 0xff) << 16 | (minor & 0xff) << 8 | (patch & 0xff);
	let v = NonZeroU32::new(v)?;

	Some((v.get(), major))
}

/// # Try Cache.
///
/// The downloaded files are cached locally in the `target` directory, but we
/// don't want to run the risk of those growing stale if they persist between
/// sessions, etc.
///
/// At the moment, cached files are used if they are less than an hour old,
/// otherwise the cache is ignored and they're downloaded fresh.
fn try_cache(path: &Path) -> Option<String> {
	std::fs::metadata(path)
		.ok()
		.filter(Metadata::is_file)
		.and_then(|meta| meta.modified().ok())
		.and_then(|time| time.elapsed().ok().filter(|secs| secs.as_secs() < 3600))
		.and_then(|_| std::fs::read_to_string(path).ok())
}
