//! `log_result` allows logging `Err` conditions without unwrapping.

#![no_std]

use core::{
    fmt::{Debug, Display},
    panic::Location,
};

/// This trait allows logging a `Result` that is `Err`
/// at the selected log level using the `Display` trait.
///
/// Returns the original result.
pub trait ResultLog {
    /// Logs at the `error` level as `Display` if `Err`.
    fn error(self) -> Self;
    /// Logs at the `warn` level as `Display` if `Err`.
    fn warn(self) -> Self;
    /// Logs at the `info` level as `Display` if `Err`.
    fn info(self) -> Self;
    /// Logs at the `debug` level as `Display` if `Err`.
    fn debug(self) -> Self;
    /// Logs at the `trace` level as `Display` if `Err`.
    fn trace(self) -> Self;
}

/// This trait allows logging a `Result` that is `Err`
/// at the selected log level using the `Debug` trait.
///
/// Returns the original result.
pub trait ResultLogDebug {
    /// Logs at the `error` level as `Debug` if `Err`.
    fn error_dbg(self) -> Self;
    /// Logs at the `warn` level as `Debug` if `Err`.
    fn warn_dbg(self) -> Self;
    /// Logs at the `info` level as `Debug` if `Err`.
    fn info_dbg(self) -> Self;
    /// Logs at the `debug` level as `Debug` if `Err`.
    fn debug_dbg(self) -> Self;
    /// Logs at the `trace` level as `Debug` if `Err`.
    fn trace_dbg(self) -> Self;
}

impl<T, E: Display> ResultLog for Result<T, E> {
    #[track_caller]
    fn error(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::error!("Err at {caller}:\n `{err}`"))
    }
    #[track_caller]
    fn warn(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::warn!("Err at {caller}:\n `{err}`"))
    }
    #[track_caller]
    fn info(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::info!("Err at {caller}:\n `{err}`"))
    }
    #[track_caller]
    fn debug(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::debug!("Err at {caller}:\n `{err}`"))
    }
    #[track_caller]
    fn trace(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::trace!("Err at {caller}:\n `{err}`"))
    }
}
impl<T, E: Debug> ResultLogDebug for Result<T, E> {
    #[track_caller]
    fn error_dbg(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::error!("Err at {caller}:\n `{err:?}`"))
    }
    #[track_caller]
    fn warn_dbg(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::warn!("Err at {caller}:\n `{err:?}`"))
    }
    #[track_caller]
    fn info_dbg(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::info!("Err at {caller}:\n `{err:?}`"))
    }
    #[track_caller]
    fn debug_dbg(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::debug!("Err at {caller}:\n `{err:?}`"))
    }
    #[track_caller]
    fn trace_dbg(self) -> Self {
        let caller = Location::caller();
        self.inspect_err(|err| log::trace!("Err at {caller}:\n `{err:?}`"))
    }
}
