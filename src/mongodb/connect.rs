use mongodb::{Client, options::ClientOptions};
pub async fn check_database() -> Result<Client, mongodb::error::Error> {

    let _mongodb_uri = "mongodb://localhost:27017".to_string();

    let mut client_options = ClientOptions::parse(_mongodb_uri).await?;

    client_options.app_name = Some("Rust Full Stack)".to_string());

    let client = Client::with_options(client_options)?;

    
    Ok(client)
}

pub async fn list_databases(client: &Client) -> Result<(), mongodb::error::Error>{
    
    println!("List of databases");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
    
}