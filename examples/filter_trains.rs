//! # Example: Filter Trains
//!
//! This example shows how to filter trains based on the route name and then determine what
//! station the train is currently in route to.
use amtrak_api::{responses::TrainStatus, Client};
use chrono::{Local, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let keystone_trains = client
        .trains()
        .await?
        .0
        .into_iter()
        .flat_map(|(_, trains)| {
            trains
                .into_iter()
                .filter(|train| train.route_name == "Keystone")
        })
        .map(|train| {
            let enroute_information = train
                .stations
                .iter()
                .find(|station| station.status == TrainStatus::Enroute)
                .map(|station| (station.name.clone(), station.arrival));

            (train, enroute_information)
        })
        .collect::<Vec<_>>();

    keystone_trains
        .iter()
        .for_each(|(train, enroute_information)| {
            if let Some((station_name, arrival)) = enroute_information {
                let time_till_arrival = if let Some(arrival) = arrival {
                    let local_now = Local::now().with_timezone(&Utc);
                    let arrival_utc = arrival.with_timezone(&Utc);

                    format!(
                        "{} minutes",
                        arrival_utc.signed_duration_since(local_now).num_minutes()
                    )
                } else {
                    "N/A".to_string()
                };

                println!(
                    "{} train is heading to {}, currently enroute to {} with an ETA of {}",
                    train.train_id, train.destination_name, station_name, time_till_arrival
                );
            } else {
                println!(
                    "{} train is heading to {}",
                    train.train_id, train.destination_code
                );
            }
        });
    Ok(())
}
