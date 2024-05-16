pub mod record_client;

use record_client::PatientRecordManager;

fn get_connection_string() -> String {
    dotenv::dotenv().ok();
    dotenv::var("connection_url").unwrap()
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let connection_uri = get_connection_string();
    
    let client = PatientRecordManager::new_with_uri(&connection_uri).await?;

    let doctor = client.register_patient("madaya", 24).await;
    println!("{:?}", doctor);
    Ok(())
}