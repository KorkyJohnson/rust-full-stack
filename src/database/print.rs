use crate::database::employee::Employee;
use clearscreen::clear;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client};

pub async fn print_database(client: &Client) -> Result<(), mongodb::error::Error> {
    clear().expect("Failed to clear the screen at add employee");

    let db = &client.database("Employee");
    let coll = db.collection::<Employee>("employees");

    let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();
    let employees: Vec<Employee> = coll
        .find(None, find_options)
        .await
        .expect("Unable to get cursor")
        .try_collect()
        .await
        .expect("Unable to get items in cursor");

    for e in employees.iter() {
        println!("Employee: {}, Title: {}", e.name, e.title);
    }

    Ok(())
}
