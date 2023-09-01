use mongodb::{Client, Database};
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