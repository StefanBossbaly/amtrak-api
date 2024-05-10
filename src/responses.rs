use std::{collections::HashMap, fmt};

use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize};

/// The response from the `/trains` or `/trains/{:train_id}` endpoint.
///
/// Each key in the hashmap is the string representation of the
/// [`train_num`] field. The value is a list of trains that have the
/// specified [`train_num`] field. I have not seen an instance where
/// multiple trains have the same [`train_num`] and therefore each list
/// in the map has only one item. It is possible for multiple trains to
/// have the same [`train_num`] so that case must be handled in the
/// client code.
///
/// [`train_num`]: Train::train_num
pub type TrainResponse = HashMap<String, Vec<Train>>;

/// The response from the `/trains` or `/trains/{:train_id}` endpoint.
///
/// We have to wrap this in a structure so that we can implement the
/// Deserialize trait for it.
#[derive(Debug, Clone)]
pub(crate) struct TrainResponseWrapper(
    /// The actual response from the Amtrak API
    pub(crate) TrainResponse,
);

impl<'de> Deserialize<'de> for TrainResponseWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TrainResponseWrapperVisitor)
    }
}

/// Custom visitor used to deserialize responses from the `/trains` or
/// `/trains/{:train_id}` endpoint.
///
/// On empty data the Amtrak API will serialize an empty vector as `[]`. On
/// normal content responses, the API will instead serialize a dictionary using
/// `{"key1", "<content>"}`. This does not place nicely with serde which
/// (rightfully) expects the type to be the same for every endpoint response. To
/// handle this discrepancy, we implement our own visitor which will handle both
/// response.
struct TrainResponseWrapperVisitor;

impl<'de> de::Visitor<'de> for TrainResponseWrapperVisitor {
    type Value = TrainResponseWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a HashMap or an empty array")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        Ok(TrainResponseWrapper(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        Ok(TrainResponseWrapper(HashMap::new()))
    }
}

/// Represents an Amtrak train
#[derive(Debug, Deserialize, Clone)]
pub struct Train {
    /// The human readable route name of this train.
    ///
    /// # Examples:
    /// * "Keystone" (for the Keystone Corridor)
    /// * "Northeast Regional" (for the Northeast Corridor)
    #[serde(rename = "routeName")]
    pub route_name: String,

    /// The (possible unique) number identifying the train.
    #[serde(rename = "trainNum")]
    pub train_num: String,

    /// The concatenation of the [`train_num`] with another number (not sure
    /// what exactly) in the format "{:train_num}-{:instance}".
    ///
    /// # Examples:
    /// * `6-4`
    /// * `93-4`
    ///
    /// [`train_num`]: Self::train_num
    #[serde(rename = "trainID")]
    pub train_id: String,

    /// The current latitude of the train
    pub lat: f64,

    /// The current longitude of the train
    pub lon: f64,

    /// The human readable status of the timelyness of the train.
    ///
    /// # Examples:
    /// * `X Minutes Early`
    /// * `X Hours, Y Minutes Early`
    /// * `X Minutes Late`
    /// * `X Hours, Y Minutes Late`
    /// * `On Time`
    /// * `Unknown`
    /// * `NaN Minutes Early` (yes really)
    #[serde(rename = "trainTimely")]
    pub train_timely: String,

    /// List of stations that the train will visit. The stations are listed in
    /// the same order the train will stop at each.
    pub stations: Vec<TrainStation>,

    /// The current compass heading of the train.
    pub heading: Heading,

    /// Unsure of what this field symbolizes.
    #[serde(rename = "eventCode")]
    pub event_code: String,

    /// Unsure of what this field symbolizes.
    #[serde(rename = "eventTZ")]
    pub event_tz: Option<String>,

    /// Unsure of what this field symbolizes.
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,

    /// The station code where the train originated from (aka the first
    /// station in this train's route).
    ///
    /// # Examples:
    /// * `PHL`
    /// * `NYP`
    #[serde(rename = "origCode")]
    pub origin_code: String,

    /// The timezone of the original station
    ///
    /// # Examples:
    /// * `America/New_York`
    /// * `America/Chicago`
    #[serde(rename = "originTZ")]
    pub origin_tz: String,

    /// The full human readable name of the station where the train originated
    /// from (aka the first station in this train's route).
    ///
    /// # Examples:
    /// * `Philadelphia 30th Street`
    /// * `New York Penn`
    #[serde(rename = "origName")]
    pub origin_name: String,

    /// The station code where the train is heading to (aka the final
    /// destination of the train).
    ///
    /// # Examples:
    /// * `PHL`
    /// * `NYP`
    #[serde(rename = "destCode")]
    pub destination_code: String,

    /// The timezone of destination station
    ///
    /// # Examples:
    /// * `America/New_York`
    /// * `America/Chicago`
    #[serde(rename = "destTZ")]
    pub destination_tz: String,

    /// The full human readable name of the station where the train is heading
    /// (aka the final destination of the train).
    ///
    /// # Examples:
    /// * `Philadelphia 30th Street`
    /// * `New York Penn`
    #[serde(rename = "destName")]
    pub destination_name: String,

    /// The current state of the train
    #[serde(rename = "trainState")]
    pub train_state: TrainState,

    /// The current velocity (in mph) of the train
    pub velocity: f32,

    /// A human readable status message.
    ///
    /// # Examples:
    /// * ` ` (Empty string, single whitespace)
    /// * `SERVICE DISRUPTION`
    #[serde(rename = "statusMsg")]
    pub status_message: String,

    /// The time at which this train entry was created. The entry will have the
    /// local timezone as a fixed offset.
    ///
    /// # Examples:
    /// * `2023-09-04T07:46:06-04:00`
    /// * `2023-09-04T07:00:00-05:00`
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,

    /// The time at which this train entry was last updated. The entry will have
    /// the local timezone as a fixed offset.
    ///
    /// # Examples:
    /// * `2023-09-04T07:46:06-04:00`
    /// * `2023-09-04T07:00:00-05:00`
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,

    /// Unsure of what this field symbolizes.
    #[serde(rename = "lastValTS")]
    pub last_value: DateTime<FixedOffset>,

    /// Unsure of what this field symbolizes.
    #[serde(rename = "objectID")]
    pub object_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrainStation {
    /// The full human readable name of the station.
    ///
    /// # Examples:
    /// * `Philadelphia 30th Street`
    /// * `New York Penn`
    pub name: String,

    /// The unique identification code of this station.
    ///
    /// # Examples:
    /// * `PHL`
    /// * `NYP`
    pub code: String,

    /// The timezone of this station.
    pub tz: String,
    pub bus: bool,

    /// The scheduled arrival time of this train for the current station.
    #[serde(rename = "schArr")]
    pub schedule_arrival: DateTime<FixedOffset>,

    /// The scheduled departure time of this train for the current station.
    #[serde(rename = "schDep")]
    pub schedule_departure: DateTime<FixedOffset>,

    /// The actual arrival time of this train for the current station specified
    /// by [`name`] or [`code`]. When the [`status`] is [`Departed`] this
    /// field shows a historical value of how late or early the train
    /// arrived. When the [`status`] is [`Enroute`] this field is a
    /// prediction on how late or early the train will arrive.
    ///
    /// Examples:
    /// `2023-09-05T16:22:00-05:00`
    /// `2023-09-05T15:54:00-05:00`
    /// `null` or not included in response
    ///
    /// [`name`]: Self::name
    /// [`code`]: Self::code
    /// [`status`]: Self::status
    /// [`Departed`]: TrainStatus::Departed
    /// [`Enroute`]: TrainStatus::Enroute
    #[serde(rename = "arr", default)]
    pub arrival: Option<DateTime<FixedOffset>>,

    /// The actual departure time of this train for the current station
    /// specified by [`name`] or [`code`]. When the [`status`] is [`Departed`]
    /// this field shows a historical value of how late or early the train
    /// departed. When the [`status`] is [`Enroute`] this field is a
    /// prediction on how late or early the train will depart.
    ///
    /// Examples:
    /// `2023-09-05T16:22:00-05:00`
    /// `2023-09-05T15:54:00-05:00`
    /// `null` or not included in response
    ///
    /// [`name`]: Self::name
    /// [`code`]: Self::code
    /// [`status`]: Self::status
    /// [`Departed`]: TrainStatus::Departed
    /// [`Enroute`]: TrainStatus::Enroute
    #[serde(rename = "dep", default)]
    pub departure: Option<DateTime<FixedOffset>>,

    /// A human readable comment on the arrival time of this train for current
    /// station specified by [`name`] or [`code`]. When the [`status`] is
    /// [`Departed`] this field shows a historical value of how late or
    /// early the train arrived. When the [`status`] is [`Enroute`] this
    /// field is a prediction on how late or early the train will arrive.
    ///
    /// Examples:
    /// `19 Minutes Late`
    /// `On Time`
    /// `NaN Minutes Early` (Yes really)
    ///
    /// [`name`]: Self::name
    /// [`code`]: Self::code
    /// [`status`]: Self::status
    /// [`Departed`]: TrainStatus::Departed
    /// [`Enroute`]: TrainStatus::Enroute
    #[serde(rename = "arrCmnt")]
    pub arrival_comment: String,

    /// A human readable comment on the departure time of this train for the
    /// current station specified by [`name`] or [`code`]. When the
    /// [`status`] is [`Departed`] this field shows a historical value of
    /// how late or early the train departed. When the [`status`] is
    /// [`Enroute`] this field is a prediction on how late or early the
    /// train will depart.
    ///
    /// Examples:
    /// `19 Minutes Late`
    /// `On Time`
    /// `NaN Minutes Early` (Yes really)
    ///
    /// [`name`]: Self::name
    /// [`code`]: Self::code
    /// [`status`]: Self::status
    /// [`Departed`]: TrainStatus::Departed
    /// [`Enroute`]: TrainStatus::Enroute
    #[serde(rename = "depCmnt")]
    pub departure_comment: String,

    /// The current status of this train for the current station specified by
    /// [`name`] or [`code`].
    pub status: TrainStatus,
}

/// Describes a train's heading using cardinal directions
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum Heading {
    /// North heading
    N,

    /// Northeast heading
    NE,

    /// East heading
    E,

    /// Southeast heading
    SE,

    /// South heading
    S,

    /// Southwest heading
    SW,

    /// West heading
    W,

    /// Northwest heading
    NW,
}

/// Represents the current status of an Amtrak train being tracked in
/// association with a [`Station`].
///
/// This status can only be applied to a combination of a [`Train`] and a
/// [`Station`]. It is referenced in the [`stations`] field.
///
/// [`Station`]: Station
/// [`Train`]: Train
/// [`stations`]: Train::stations
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum TrainStatus {
    /// The train has not yet arrived at the specified station.
    Enroute,

    /// The train is currently at the specified station.
    Station,

    /// The train has already arrived at departed from teh specified station.
    Departed,

    /// The status of the train is unknown
    Unknown,
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum TrainState {
    /// The train is awaiting departure from its origin station
    Predeparture,

    /// The train is currently on its route.
    Active,

    /// The train has completed its journey is not longer servicing its route.
    Completed,
}

/// The response from the `/stations` or `/stations/{:station_code}` endpoint.
///
/// Each key in the hashmap is the unique station code which will match the
/// [`code`] field. The value is the [`Station`] structure that is
/// associated with the unique station [`code`].
///
/// [`code`]: Station::code
/// [`Station`]: Station
pub type StationResponse = HashMap<String, Station>;

/// The response from the `/stations` or `/stations/{:station_code}` endpoint.
///
/// We have to wrap this in a structure so that we can implement the
/// Deserialize trait for it.
#[derive(Debug, Clone)]
pub(crate) struct StationResponseWrapper(
    /// The actual response from the Amtrak API
    pub(crate) HashMap<String, Station>,
);

impl<'de> Deserialize<'de> for StationResponseWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(StationResponseWrapperVisitor)
    }
}

/// Custom visitor used to deserialize responses from the `/stations` or
/// `/stations/{:station_code}` endpoint.
///
/// On empty data the Amtrak API will serialize an empty vector as `[]`. On
/// normal content responses, the API will instead serialize a dictionary using
/// `{"key1", "<content>"}`. This does not place nicely with serde which
/// (rightfully) expects the type to be the same for every endpoint response. To
/// handle this discrepancy, we implement our own visitor which will handle both
/// response.
struct StationResponseWrapperVisitor;

impl<'de> de::Visitor<'de> for StationResponseWrapperVisitor {
    type Value = StationResponseWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a HashMap or an empty array")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        Ok(StationResponseWrapper(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        Ok(StationResponseWrapper(HashMap::new()))
    }
}

/// Represents a unique station that Amtrak services
#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    /// The full human readable name of the station.
    ///
    /// # Examples:
    /// * `Philadelphia 30th Street`
    /// * `New York Penn`
    #[serde(default)]
    pub name: String,

    /// The unique identification code of this station.
    ///
    /// # Examples:
    /// * `PHL`
    /// * `NYP`
    pub code: String,

    /// The timezone of the station
    ///
    /// # Examples:
    /// * `America/New_York`
    /// * `America/Chicago`
    #[serde(default)]
    pub tz: String,

    /// The latitude of the station
    pub lat: f64,

    /// The longitude of the station
    pub lon: f64,

    /// The first address line of the stations
    ///
    /// # Examples:
    /// * `2955 Market Street`
    /// * `351 West 31st Street`
    pub address1: String,

    /// The second address line of the station
    pub address2: String,

    /// The city of the station
    ///
    /// # Examples:
    /// * `Philadelphia`
    /// * `New York`
    pub city: String,

    /// The two character abbreviation of the state of the station
    ///
    /// # Examples:
    /// * `PA`
    /// * `NY`
    pub state: String,

    /// The zip code of the station
    ///
    /// # Examples:
    /// * `19104`
    /// * `10001`
    pub zip: u32,

    /// A list of current [`train_id`] that have departed from or are enroute to
    /// this station.
    ///
    /// [`train_id`]: Train::train_id
    pub trains: Vec<String>,
}
