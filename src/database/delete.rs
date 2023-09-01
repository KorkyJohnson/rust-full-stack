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

    println!("Please enter employee name to delete:");
    io::stdin()
        .read_line(&mut employee.name)
        .expect("You didn't enter an employee name");
    employee.name = employee.name.trim().to_string();

    coll.delete_one(doc! {"name" : &employee.name}, None)
        .await
        .expect("Unable to delete name");

    println!("Deleted: {:?}", employee.name);

    Ok(())
}
