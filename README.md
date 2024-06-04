# AMTRAK-API ![crates.io](https://img.shields.io/crates/v/amtrak-api.svg) [![](https://docs.rs/amtrak-api/badge.svg)](https://docs.rs/amtrak-api)

This project provides developers with a standard and ergonomic Rust API for
calling the various endpoints in Amtrak Train API. It handles the serialization
and deserialization of the requests and responses and allows the developer to
use the provided well-defined data types. It also handles some of the messy
parts of the API (multiple serializations per stop, quarky endpoint responses,
multiple datetime formats, etc).

## Example Usage

```rust
//! # Example: Filter Trains
//!
//! This example shows how to filter trains based on the route name and then
//! determine what station the train is currently in route to.
use amtrak_api::{Client, TrainStatus};
use chrono::{Local, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Client::new()
        .trains()
        .await?
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
```

## Example Output

```
664-27 train is heading to New York Penn, currently enroute to Harrisburg with an ETA of 14 minutes
663-27 train is heading to Harrisburg, currently enroute to Newark Penn with an ETA of 10 minutes
660-27 train is heading to New York Penn, currently enroute to Philadelphia 30th Street with an ETA of -6 minutes
611-27 train is heading to Harrisburg, currently enroute to Harrisburg with an ETA of -1 minutes
661-27 train is heading to Harrisburg, currently enroute to Ardmore with an ETA of 4 minutes
662-27 train is heading to New York Penn, currently enroute to Parkesburg with an ETA of 2 minutes
```

## Features

- `serde_debugging` (Disabled by default): Enables the the following functions:
  `trains_with_debugging`, `train_with_debugging`, `stations_with_debugging`,
  `station_with_debugging`. These functions will operate the exact same way as
  their counterparts with the exception that deserialization is completed using
  [`serde_path_to_error`](https://crates.io/crates/serde_path_to_error) adapter.
  This crate will print out the path of the offending field if deserialization
  were to fail. The only quarky behavior is that instead of returning
  `Error::DeserializeFailed`, the debugging functions will instead return
  `Error:Other`. This is so that we can include the JSON response in the error
  as well as the path to the field that caused the deserialization to fail,
  making debugging a lot easier.

## Authors

Stefan Bossbaly

## License

This project is licensed under the MIT License - see the LICENSE file for
details

## Acknowledgments

- [Amtrak](https://amtrak.com/)
- [Amtrak API](https://api-v3.amtraker.com/v3/)
