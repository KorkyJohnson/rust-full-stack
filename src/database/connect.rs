use mongodb::{options::ClientOptions, Client};
pub async fn check_database() -> Result<Client, mongodb::error::Error> {
    let _db_name = "Employees";
    let _coll_name = "employee";

    let _mongodb_uri = "mongodb://localhost:27017".to_string();

    let mut client_options = ClientOptions::parse(_mongodb_uri).await?;

    client_options.app_name = Some("Rust Full Stack)".to_string());

    let client = Client::with_options(client_options)?;

    let db = client.database(_db_name);
    db.create_collection(_coll_name, None).await.unwrap();

    Ok(client)
}
