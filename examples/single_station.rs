//! # Example: Single Station
//!
//! This example shows how to query for a single station based on the unique
//! station code and then print out all trains that either have departed that
//! station or are currently enroute to that station.
use amtrak_api::Client;

const STATION_CODE: &str = "PHL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Client::new()
        .station(STATION_CODE)
        .await?
        .values()
        .for_each(|station| {
            println!(
                "Current train scheduled for station \"{}\": {}",
                station.name,
                station.trains.join(", ")
            );
        });

    Ok(())
}
