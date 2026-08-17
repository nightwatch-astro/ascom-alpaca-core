//! `ConformU` test harness -- thin HTTP server for ASCOM conformance testing.
//!
//! Gated behind the `conformu` feature flag. Not included in production builds.
//!
//! Mock implementations use `unwrap()` on `Mutex::lock()` throughout because
//! poisoned mutexes indicate a bug in the test harness, not a recoverable error.

#[allow(
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::too_many_lines,
    missing_docs
)]
pub mod dispatch;
#[allow(clippy::unwrap_used, missing_docs)]
pub mod management;
#[allow(
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    missing_docs
)]
pub mod mocks;
