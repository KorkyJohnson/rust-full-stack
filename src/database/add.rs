use crate::database::employee::Employee;
use clearscreen::clear;
use mongodb::Client;
use std::io;

pub async fn add_employee(client: &Client) -> Result<(), mongodb::error::Error> {
    clear().expect("Failed to clear the screen at add employee");

    let db = &client.database("Employee");
    let coll = db.collection::<Employee>("employees");
    let mut employee = Employee {
        name: String::new(),
        title: String::new(),
        salary: None,
        phone: None,
    };

    println!("Please enter employee name to add:");
    io::stdin()
        .read_line(&mut employee.name)
        .expect("You didn't enter an employee name");
    employee.name = employee.name.trim().to_string();

    println!("Please enter employee title");
    io::stdin()
        .read_line(&mut employee.title)
        .expect("You didn't enter an employee title");
    employee.title = employee.title.trim().to_string();

    let doc = vec![employee];

    coll.insert_many(doc, None).await?;
    println!("Record inserted");

    Ok(())
}
