use amtrak_api::Client;
use mockito::Server;

#[tokio::test]
async fn test_single_station() -> Result<(), amtrak_api::Error> {
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

    assert_eq!(response.len(), 1);

    let station = response.get("ABE").unwrap();
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
async fn test_canadian_station() -> Result<(), amtrak_api::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/stations")
        .with_body(
            r#"
{
    "AST": {
        "name": "Aldershot",
        "code": "AST",
        "tz": "America/Toronto",
        "lat": 43.313413,
        "lon": -79.855712,
        "address1": "1199 Waterdown Road",
        "address2": " ",
        "city": "Aldershot",
        "state": "ON",
        "zip": "L7T 4A8",
        "trains": []
    }
}"#,
        )
        .create_async()
        .await;

    let client = Client::with_base_url(server.url().as_str());
    let response = client.stations().await?;

    assert_eq!(response.len(), 1);

    let station = response.get("AST").unwrap();
    assert_eq!(station.name, "Aldershot");
    assert_eq!(station.code, "AST");
    assert_eq!(station.lat, 43.313413);
    assert_eq!(station.lon, -79.855712);
    assert_eq!(station.address1, "1199 Waterdown Road");
    assert_eq!(station.address2, " ");
    assert_eq!(station.city, "Aldershot");
    assert_eq!(station.state, "ON");
    assert_eq!(station.zip, "L7T 4A8");
    assert_eq!(station.trains.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_empty_station() -> Result<(), amtrak_api::Error> {
    let mut server = Server::new_async().await;
    let mock_server = server
        .mock("GET", "/stations/ABC")
        .with_body("[]")
        .create_async()
        .await;
    let client = Client::with_base_url(server.url().as_str());
    let response = client.station("ABC").await?;

    assert_eq!(response.len(), 0);

    mock_server.assert_async().await;

    Ok(())
}
