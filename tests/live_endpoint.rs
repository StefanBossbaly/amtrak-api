use amtrak_api::Client;

#[tokio::test]
async fn test_live_train_api() -> Result<(), amtrak_api::errors::Error> {
    let client = Client::new();
    let response = client.trains().await?;

    for (train_num, _) in response.iter() {
        client.train(train_num).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_live_station_api() -> Result<(), amtrak_api::errors::Error> {
    let client = Client::new();
    let response = client.stations().await?;

    for (station_code, _) in response.iter() {
        client.station(station_code).await?;
    }

    Ok(())
}
