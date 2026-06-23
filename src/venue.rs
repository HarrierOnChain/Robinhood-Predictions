//! Robinhood Predictions venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Robinhood Predictions";

/// Venue category.
pub const VENUE_TYPE: &str = "Brokerage-integrated";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Directional Arbitrage",
    "Sports Execution",
];
