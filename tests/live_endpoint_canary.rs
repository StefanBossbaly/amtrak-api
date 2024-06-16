#![cfg(feature = "serde_debugging")]
use amtrak_api::Client;

/// Test the live train endpoint using serde_path_to_error as the deserialize driver
///
/// This test will call the live train endpoint to list all the trains that are currently
/// in the system. We do not test for correct deserialization since we do not have truth
/// data to compare against, we are just ensuring that we can deserialize the response
/// provided by the Amtrak API.
#[tokio::test]
async fn test_live_train_api() -> anyhow::Result<()> {
    let client = Client::new();
    let _ = client.trains_with_debugging().await?;

    Ok(())
}

/// Test the live station endpoint using serde_path_to_error as the deserialize driver
///
/// This test will call the live station endpoint to list all the stations that are currently
/// in the system. We do not test for correct deserialization since we do not have truth
/// data to compare against, we are just ensuring that we can deserialize the response
/// provided by the Amtrak API.
#[tokio::test]
async fn test_live_station_api() -> anyhow::Result<()> {
    let client = Client::new();
    let _ = client.stations_with_debugging().await?;

    Ok(())
}
