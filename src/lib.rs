//! # Amtrak Rust API
//!
//! Amtrak Rust API allows the caller to query the Amtrak API for information
//! about trains and stations in its network.
//!
//! Please check [`Client`] for the various endpoints this API allows you to
//! call into.
//!
//! Note: This library is not affiliated with Amtrak in any way and is an
//! unofficial implementation of the public facing API. Amtrak is a registered
//! trademark of the National Railroad Passenger Corporation.
//!
//! # Example usage
//! Here is an example of querying trains that have stopped at or will stop at a
//! specific station.
//!
//! ```rust
//! use amtrak_api::Client;
//!
//! const STATION_CODE: &str = "PHL";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     Client::new()
//!         .station(STATION_CODE)
//!         .await?
//!         .values()
//!         .for_each(|station| {
//!             println!(
//!                 "Current train scheduled for station \"{}\": {}",
//!                 station.name,
//!                 station.trains.join(", ")
//!             );
//!         });
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod errors;
pub mod responses;

pub use client::Client;
