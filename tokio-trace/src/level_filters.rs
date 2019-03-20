use std::cmp::Ordering;
use tokio_trace_core::Level;

/// Describes the level of verbosity of a span or event.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LevelFilter(Option<Level>);

impl LevelFilter {
    /// The "off" level.
    ///
    /// Designates to turn off logging.
    pub const OFF: LevelFilter = LevelFilter(None);
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: LevelFilter = LevelFilter(Some(Level::ERROR));
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: LevelFilter = LevelFilter(Some(Level::WARN));
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: LevelFilter = LevelFilter(Some(Level::INFO));
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: LevelFilter = LevelFilter(Some(Level::DEBUG));
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: LevelFilter = LevelFilter(Some(Level::TRACE));
}

impl PartialEq<LevelFilter> for Level {
    fn eq(&self, other: &LevelFilter) -> bool {
        match other.0 {
            None => false,
            Some(ref level) => self.eq(level),
        }
    }
}

impl PartialOrd<LevelFilter> for Level {
    fn partial_cmp(&self, other: &LevelFilter) -> Option<Ordering> {
        match other.0 {
            None => Some(Ordering::Less),
            Some(ref level) => self.partial_cmp(level),
        }
    }
}

/// The statically resolved maximum trace level.
///
/// See the crate level documentation for information on how to configure this.
///
/// This value is checked by the `event` macro. Code that manually calls functions on that value
/// should compare the level against this value.
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::OFF;

#[cfg(feature = "max_level_off")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::OFF;

#[cfg(feature = "max_level_error")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::ERROR;

#[cfg(feature = "max_level_warn")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::WARN;

#[cfg(feature = "max_level_info")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::INFO;

#[cfg(feature = "max_level_debug")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::DEBUG;

#[cfg(feature = "max_level_trace")]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::TRACE;

#[cfg(all(not(debug_assertions), feature = "release_max_level_off"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::OFF;

#[cfg(all(not(debug_assertions), feature = "release_max_level_error"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::ERROR;

#[cfg(all(not(debug_assertions), feature = "release_max_level_warn"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::WARN;

#[cfg(all(not(debug_assertions), feature = "release_max_level_info"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::INFO;

#[cfg(all(not(debug_assertions), feature = "release_max_level_debug"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::DEBUG;

#[cfg(all(not(debug_assertions), feature = "release_max_level_trace"))]
pub const STATIC_MAX_LEVEL: LevelFilter = LevelFilter::TRACE;
