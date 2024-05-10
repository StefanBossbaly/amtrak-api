//! # Example: Single Trains
//!
//! This example shows how to query for a single train based on the unique train
//! id and then determine if it has stopped at 30th street station yet.
use amtrak_api::{Client, TrainStatus};

const TRAIN_ID: &str = "612-5";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Attempt to query the status of the "612-5" train
    let response = client.train(TRAIN_ID).await?;
    let train_612_5 = response.get(TRAIN_ID);

    match train_612_5 {
        Some(trains) => match trains.len() {
            1 => {
                let phl_station = trains
                    .get(0)
                    .unwrap()
                    .stations
                    .iter()
                    .find(|station| station.code == "PHL");

                match phl_station {
                    Some(phl_station) => match phl_station.status {
                        TrainStatus::Enroute => {
                            println!("Train is enroute to Philadelphia station")
                        }
                        TrainStatus::Station => {
                            println!("Train is current at Philadelphia station")
                        }
                        TrainStatus::Departed => {
                            println!("Train has departed Philadelphia station")
                        }
                        TrainStatus::Unknown => println!("The train status is unknown"),
                    },
                    None => println!(
                        "Philadelphia station was not found in the \"{}\" route",
                        TRAIN_ID
                    ),
                }
            }
            0 => println!("Train \"{}\" response was empty", TRAIN_ID),
            _ => println!("More than one train returned for \"{}\"", TRAIN_ID),
        },
        None => println!(
            "Train \"{}\" is not currently in the Amtrak network",
            TRAIN_ID
        ),
    }

    Ok(())
}
