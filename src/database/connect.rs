use mongodb::{options::ClientOptions, Client, Database};
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

pub async fn list_databases(client: &Client) -> Result<(), mongodb::error::Error> {
    println!("List of databases");

    // List all the databases found
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}

pub async fn list_collections(db: &Database) -> Result<(), mongodb::error::Error> {

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    Ok(())
}
