use amtrak_api::{responses::TrainStatus, Client};
use chrono::{FixedOffset, NaiveDate};
use mockito::Server;

#[tokio::test]
async fn test_single_train() -> Result<(), amtrak_api::errors::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/trains")
        .with_body(
            r#"
{
    "657": [
        {
            "routeName": "Keystone",
            "trainNum": "657",
            "trainID": "657-30",
            "lat": 40.14815944794739,
            "lon": -76.61796031144218,
            "trainTimely": "NaN Minutes Early",
            "stations": [
            {
                "name": "New York Penn",
                "code": "NYP",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T20:30:00-04:00",
                "schDep": "2023-08-29T20:30:00-04:00",
                "arr": "2023-08-29T20:30:00-04:00",
                "dep": "2023-08-29T20:30:00-04:00",
                "arrCmnt": "0 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Newark Penn",
                "code": "NWK",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T20:45:00-04:00",
                "schDep": "2023-08-29T20:47:00-04:00",
                "arr": "2023-08-29T20:42:00-04:00",
                "dep": "2023-08-29T20:47:00-04:00",
                "arrCmnt": "3 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Trenton",
                "code": "TRE",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T21:23:00-04:00",
                "schDep": "2023-08-29T21:24:00-04:00",
                "arr": "2023-08-29T21:17:00-04:00",
                "dep": "2023-08-29T21:24:00-04:00",
                "arrCmnt": "6 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Cornwells Heights",
                "code": "CWH",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T21:35:00-04:00",
                "schDep": "2023-08-29T21:36:00-04:00",
                "arr": "2023-08-29T21:35:00-04:00",
                "dep": "2023-08-29T21:36:00-04:00",
                "arrCmnt": "0 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Philadelphia North",
                "code": "PHN",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T21:47:00-04:00",
                "schDep": "2023-08-29T21:47:00-04:00",
                "arr": "2023-08-29T21:47:00-04:00",
                "dep": "2023-08-29T21:48:00-04:00",
                "arrCmnt": "0 Minutes Early",
                "depCmnt": "On Time",
                "status": "Departed"
            },
            {
                "name": "Philadelphia 30th Street",
                "code": "PHL",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T21:55:00-04:00",
                "schDep": "2023-08-29T22:05:00-04:00",
                "arr": "2023-08-29T21:56:00-04:00",
                "dep": "2023-08-29T22:05:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Ardmore",
                "code": "ARD",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:17:00-04:00",
                "schDep": "2023-08-29T22:18:00-04:00",
                "arr": "2023-08-29T22:18:00-04:00",
                "dep": "2023-08-29T22:18:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Paoli",
                "code": "PAO",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:29:00-04:00",
                "schDep": "2023-08-29T22:30:00-04:00",
                "arr": "2023-08-29T22:31:00-04:00",
                "dep": "2023-08-29T22:32:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "On Time",
                "status": "Departed"
            },
            {
                "name": "Exton",
                "code": "EXT",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:37:00-04:00",
                "schDep": "2023-08-29T22:38:00-04:00",
                "arr": "2023-08-29T22:39:00-04:00",
                "dep": "2023-08-29T22:40:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "On Time",
                "status": "Departed"
            },
            {
                "name": "Downingtown",
                "code": "DOW",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:42:00-04:00",
                "schDep": "2023-08-29T22:42:00-04:00",
                "arr": "2023-08-29T22:46:00-04:00",
                "dep": "2023-08-29T22:47:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "5 Minutes Late",
                "status": "Departed"
            },
            {
                "name": "Coatesville",
                "code": "COT",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:49:00-04:00",
                "schDep": "2023-08-29T22:49:00-04:00",
                "arr": "2023-08-29T22:52:00-04:00",
                "dep": "2023-08-29T22:52:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "On Time",
                "status": "Departed"
            },
            {
                "name": "Parkesburg",
                "code": "PAR",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T22:55:00-04:00",
                "schDep": "2023-08-29T22:55:00-04:00",
                "arr": "2023-08-29T22:57:00-04:00",
                "dep": "2023-08-29T22:58:00-04:00",
                "arrCmnt": "On Time",
                "depCmnt": "On Time",
                "status": "Departed"
            },
            {
                "name": "Lancaster",
                "code": "LNC",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T23:17:00-04:00",
                "schDep": "2023-08-29T23:18:00-04:00",
                "arr": "2023-08-29T23:16:00-04:00",
                "dep": "2023-08-29T23:18:00-04:00",
                "arrCmnt": "1 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Mount Joy",
                "code": "MJY",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T23:28:00-04:00",
                "schDep": "2023-08-29T23:28:00-04:00",
                "arr": "2023-08-29T23:26:00-04:00",
                "dep": "2023-08-29T23:28:00-04:00",
                "arrCmnt": "2 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Elizabethtown",
                "code": "ELT",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T23:35:00-04:00",
                "schDep": "2023-08-29T23:35:00-04:00",
                "arr": "2023-08-29T23:33:00-04:00",
                "dep": "2023-08-29T23:35:00-04:00",
                "arrCmnt": "2 Minutes Early",
                "depCmnt": "0 Minutes Early",
                "status": "Departed"
            },
            {
                "name": "Middletown",
                "code": "MID",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T23:42:00-04:00",
                "schDep": "2023-08-29T23:42:00-04:00",
                "arr": "2023-08-29T23:42:00-04:00",
                "dep": "2023-08-29T23:42:00-04:00",
                "arrCmnt": "NaN Minutes Early",
                "depCmnt": "NaN Minutes Early",
                "status": "Enroute"
            },
            {
                "name": "Harrisburg",
                "code": "HAR",
                "tz": "America/New_York",
                "bus": false,
                "schArr": "2023-08-29T23:56:00-04:00",
                "schDep": "2023-08-29T23:56:00-04:00",
                "arr": null,
                "arrCmnt": "NaN Minutes Early",
                "depCmnt": "NaN Minutes Early",
                "status": "Station"
            }
            ],
            "heading": "W",
            "eventCode": "MID",
            "eventTZ": "America/New_York",
            "eventName": "Middletown",
            "origCode": "NYP",
            "originTZ": "America/New_York",
            "origName": "New York Penn",
            "destCode": "HAR",
            "destTZ": "America/New_York",
            "destName": "Harrisburg",
            "trainState": "Active",
            "velocity": 51.2444686889648,
            "statusMsg": " ",
            "createdAt": "2023-08-29T23:39:50-04:00",
            "updatedAt": "2023-08-29T23:39:50-04:00",
            "lastValTS": "2023-08-29T23:39:34-04:00",
            "objectID": 847
        }
        ]
}"#,
        )
        .create_async()
        .await;

    let client = Client::with_base_url(server.url().as_str());
    let response = client.trains().await?;

    let trains = response.get("657").unwrap();

    assert_eq!(trains.len(), 1);

    let train = trains.get(0).unwrap();

    assert_eq!(train.route_name, "Keystone");
    assert_eq!(train.train_num, "657");
    assert_eq!(train.lat, 40.14815944794739);
    assert_eq!(train.lon, -76.61796031144218);
    assert_eq!(train.train_timely, "NaN Minutes Early");

    assert_eq!(train.stations.len(), 17);

    // Check the first station
    assert_eq!(train.stations[0].name, "New York Penn");
    assert_eq!(train.stations[0].code, "NYP");
    assert_eq!(train.stations[0].tz, "America/New_York");
    assert!(!train.stations[0].bus);
    assert_eq!(train.stations[0].schedule_arrival, {
        let tz = FixedOffset::east_opt(-4 * 3600).unwrap();
        NaiveDate::from_ymd_opt(2023, 8, 29)
            .unwrap()
            .and_hms_opt(20, 30, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap()
    });
    assert_eq!(train.stations[0].schedule_departure, {
        let tz = FixedOffset::east_opt(-4 * 3600).unwrap();
        NaiveDate::from_ymd_opt(2023, 8, 29)
            .unwrap()
            .and_hms_opt(20, 30, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap()
    });
    assert_eq!(train.stations[0].arrival.unwrap(), {
        let tz = FixedOffset::east_opt(-4 * 3600).unwrap();
        NaiveDate::from_ymd_opt(2023, 8, 29)
            .unwrap()
            .and_hms_opt(20, 30, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap()
    });
    assert_eq!(train.stations[0].departure.unwrap(), {
        let tz = FixedOffset::east_opt(-4 * 3600).unwrap();
        NaiveDate::from_ymd_opt(2023, 8, 29)
            .unwrap()
            .and_hms_opt(20, 30, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap()
    });
    assert_eq!(train.stations[0].arrival_comment, "0 Minutes Early");
    assert_eq!(train.stations[0].departure_comment, "0 Minutes Early");
    assert_eq!(train.stations[0].status, TrainStatus::Departed);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_empty_trains() -> Result<(), amtrak_api::errors::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/trains")
        .with_body("[]")
        .create_async()
        .await;
    let client = Client::with_base_url(server.url().as_str());
    let response = client.trains().await?;

    assert_eq!(response.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}
