use amtrak_api::Client;
use mockito::Server;

#[tokio::test]
async fn test_single_station() -> Result<(), amtrak_api::errors::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/stations")
        .with_body(
            r#"
{
    "ABE": {
        "name": "Aberdeen",
        "code": "ABE",
        "tz": "America/New_York",
        "lat": 39.508447,
        "lon": -76.16326,
        "address1": "18 East Bel Air Avenue",
        "address2": " ",
        "city": "Aberdeen",
        "state": "MD",
        "zip": "21001",
        "trains": []
    }
}"#,
        )
        .create_async()
        .await;

    let client = Client::with_base_url(server.url().as_str());
    let response = client.stations().await?;

    assert_eq!(response.0.len(), 1);

    let station = response.0.get("ABE").unwrap();
    assert_eq!(station.name, "Aberdeen");
    assert_eq!(station.code, "ABE");
    assert_eq!(station.lat, 39.508447);
    assert_eq!(station.lon, -76.16326);
    assert_eq!(station.address1, "18 East Bel Air Avenue");
    assert_eq!(station.address2, " ");
    assert_eq!(station.city, "Aberdeen");
    assert_eq!(station.state, "MD");
    assert_eq!(station.zip, "21001");
    assert_eq!(station.trains.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_empty_station() -> Result<(), amtrak_api::errors::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/stations/ABC")
        .with_body("[]")
        .create_async()
        .await;
    let client = Client::with_base_url(server.url().as_str());
    let response = client.station("ABC").await?;

    assert_eq!(response.0.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}
