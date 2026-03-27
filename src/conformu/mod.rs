//! ConformU test harness — thin HTTP server for ASCOM conformance testing.
//!
//! Gated behind the `conformu` feature flag. Not included in production builds.

pub mod dispatch;
pub mod management;
pub mod mocks;
