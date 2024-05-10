//! # Example: Filter Stations
//!
//! This example shows how to filter trains based on the station's state.
use amtrak_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Client::new()
        .stations()
        .await?
        .values()
        .filter(|station| station.state == "PA")
        .for_each(|station| {
            println!("Station \"{}\" is in PA", station.name);
        });

    Ok(())
}
