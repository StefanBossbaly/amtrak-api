use crate::{errors, responses};

const BASE_API_URL: &str = "https://api-v3.amtraker.com/v3";

pub type Result<T> = std::result::Result<T, errors::Error>;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a new instance with the default Amtrak API endpoint
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            base_url: BASE_API_URL.to_string(),
        }
    }

    /// Creates a new instance with the provided Amtrak endpoint
    ///
    /// This function is useful for testing since Mockito will create a local
    /// endpoint
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base url of the endpoint that this client will query
    ///   when making API calls.
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::with_base_url("https://api-v3.amtraker.com/v3");
    ///     Ok(())
    /// }
    /// ```
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Returns all trains being tracked by Amtrak
    ///
    /// This function calls into the `/trains` endpoint.
    ///
    /// This function will list all current trains being tracked by the Amtrak
    /// API.
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::{responses::TrainStatus, Client};
    /// use chrono::{Local, Utc};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///
    ///     // Filter out any trains which route is not a part of the Keystone Corridor
    ///     let keystone_trains = client
    ///         .trains()
    ///         .await?
    ///         .0
    ///         .into_iter()
    ///         .flat_map(|(_, trains)| {
    ///             trains
    ///                 .into_iter()
    ///                 .filter(|train| train.route_name == "Keystone")
    ///         })
    ///         .map(|train| {
    ///             // The stations for each train are listed in order in which they will be visited.
    ///             // Find the first station which has the Enroute status which indicates it is the next stop on the route.
    ///             let enroute_information = train
    ///                 .stations
    ///                 .iter()
    ///                 .find(|station| station.status == TrainStatus::Enroute)
    ///                 .map(|station| (station.name.clone(), station.arrival));
    ///
    ///             (train, enroute_information)
    ///         })
    ///         .collect::<Vec<_>>();
    ///
    ///     keystone_trains
    ///         .iter()
    ///         .for_each(|(train, enroute_information)| {
    ///             // Ensure that we did find a enroute station for this train
    ///             if let Some((station_name, arrival)) = enroute_information {
    ///                 let time_till_arrival = if let Some(arrival) = arrival {
    ///                     // Figure out the amount of time between when the train is suppose to arrive
    ///                     // vs what the current time is. Ensure that we account for timezone by converting
    ///                     // both to Utc.
    ///                     let local_now = Local::now().with_timezone(&Utc);
    ///                     let arrival_utc = arrival.with_timezone(&Utc);
    ///
    ///                     format!(
    ///                         "{} minutes",
    ///                         arrival_utc.signed_duration_since(local_now).num_minutes()
    ///                     )
    ///                 } else {
    ///                     "N/A".to_string()
    ///                 };
    ///
    ///                 println!(
    ///                     "{} train is heading to {}, currently enroute to {} with an ETA of {}",
    ///                     train.train_id, train.destination_name, station_name, time_till_arrival
    ///                 );
    ///             } else {
    ///                 println!(
    ///                     "{} train is heading to {}",
    ///                     train.train_id, train.destination_code
    ///                 );
    ///             }
    ///         });
    ///     Ok(())
    /// }
    /// ```
    pub async fn trains(&self) -> Result<responses::TrainResponse> {
        let url = format!("{}/trains", self.base_url);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::TrainResponse>()
            .await?;

        Ok(response)
    }

    /// Returns the specified train(s) being tracked by Amtrak
    ///
    /// This function calls into the `/trains/{:train_id}` endpoint.
    ///
    /// This function will list the specified train being tracked by the Amtrak
    /// API.
    ///
    /// # Arguments
    ///
    /// * `train_identifier` - Can either be the [`train_id`] or the
    ///   [`train_num`] of the train the caller wants to query.
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::{responses::TrainStatus, Client};
    ///
    /// const TRAIN_ID: &str = "612-5";
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///
    ///     // Attempt to query the status of the "612-5" train
    ///     let response = client.train(TRAIN_ID).await?;
    ///     let train_612_5 = response.0.get(TRAIN_ID);
    ///
    ///     match train_612_5 {
    ///         Some(trains) => match trains.len() {
    ///             1 => {
    ///                 let phl_station = trains
    ///                     .get(0)
    ///                     .unwrap()
    ///                     .stations
    ///                     .iter()
    ///                     .find(|station| station.code == "PHL");
    ///
    ///                 match phl_station {
    ///                     Some(phl_station) => match phl_station.status {
    ///                         TrainStatus::Enroute => {
    ///                             println!("Train is enroute to Philadelphia station")
    ///                         }
    ///                         TrainStatus::Station => {
    ///                             println!("Train is current at Philadelphia station")
    ///                         }
    ///                         TrainStatus::Departed => {
    ///                             println!("Train has departed Philadelphia station")
    ///                         }
    ///                         TrainStatus::Unknown => println!("The train status is unknown"),
    ///                     },
    ///                     None => println!(
    ///                         "Philadelphia station was not found in the \"{}\" route",
    ///                         TRAIN_ID
    ///                     ),
    ///                 }
    ///             }
    ///             0 => println!("Train \"{}\" response was empty", TRAIN_ID),
    ///             _ => println!("More than one train returned for \"{}\"", TRAIN_ID),
    ///         },
    ///         None => println!(
    ///             "Train \"{}\" is not currently in the Amtrak network",
    ///             TRAIN_ID
    ///         ),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`train_id`]: responses::Train::train_id
    /// [`train_num`]: responses::Train::train_num
    pub async fn train(&self, train_identifier: &str) -> Result<responses::TrainResponse> {
        let url = format!("{}/trains/{}", self.base_url, train_identifier);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::TrainResponse>()
            .await?;

        Ok(response)
    }

    /// Returns all the stations in the Amtrak network
    ///
    /// This function calls into the `/stations` endpoint.
    ///
    /// This function will list all the stations in the Amtrak network.
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     Client::new()
    ///         .stations()
    ///         .await?
    ///         .0
    ///         .values()
    ///         .filter(|station| station.state == "PA")
    ///         .for_each(|station| {
    ///             println!("Station \"{}\" is in PA", station.name);
    ///         });
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn stations(&self) -> Result<responses::StationResponse> {
        let url = format!("{}/stations", self.base_url);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::StationResponse>()
            .await?;

        Ok(response)
    }

    /// Returns the specified station in the Amtrak network
    ///
    /// This function calls into the `/stations/{:station_code}` endpoint.
    ///
    /// This function will query the station with the provided `station_code`.
    ///
    /// # Arguments
    ///
    /// * `station_code` - The station [`code`] the caller wants to query.
    ///
    /// # Example
    ///
    /// ```rust
    /// use amtrak_api::Client;
    ///
    /// const STATION_CODE: &str = "PHL";
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     Client::new()
    ///         .station(STATION_CODE)
    ///         .await?
    ///         .0
    ///         .values()
    ///         .for_each(|station| {
    ///             println!(
    ///                 "Current train scheduled for station \"{}\": {}",
    ///                 station.name,
    ///                 station.trains.join(", ")
    ///             );
    ///         });
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`code`]: responses::TrainStation::code
    pub async fn station(&self, station_code: &str) -> Result<responses::StationResponse> {
        let url = format!("{}/stations/{}", self.base_url, station_code);

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json::<responses::StationResponse>()
            .await?;

        Ok(response)
    }
}
