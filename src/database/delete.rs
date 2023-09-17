use crate::database::employee::Employee;
use clearscreen::clear;
use mongodb::{bson::doc, Client};
use std::io;

pub async fn delete_employee(client: &Client) -> Result<(), mongodb::error::Error> {
    clear().expect("Failed to clear the screen at delete employee");

    let db = &client.database("Employee");
    let coll = db.collection::<Employee>("employees");

    let mut employee = Employee {
        name: String::new(),
        title: String::new(),
        salary: None,
        phone: None,
    };

    let mut search_pattern = String::new();

    println!("Please enter employee name to delete:");
    io::stdin()
        // .read_line(&mut employee.name)
        .read_line(&mut search_pattern)
        .expect("You didn't enter an employee name");
    employee.name = employee.name.trim().to_string();
    search_pattern = search_pattern.trim().to_string();

    let query = doc! {"name": &search_pattern};

    // coll.delete_one(doc! {"name" : &employee.name}, None)
    let result = coll
        .delete_one(query, None)
        .await
        .expect("Unable to delete name");

    if result.deleted_count > 0 {
        println!("Deleted: {}", search_pattern);
    } else {
        println!("No such record exists, nothing deleted")
    };

    Ok(())
}
