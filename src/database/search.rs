use crate::database::employee::Employee;
use clearscreen::clear;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, options::FindOptions};
use std::io;

pub async fn search_employees(client: &Client) -> Result<(), mongodb::error::Error> {
    clear().expect("Failed to clear the screen at delete employee");

    let db = &client.database("Employee");
    let coll = db.collection::<Employee>("employees");

    let mut employee = Employee {
        name: String::new(),
        title: String::new(),
        salary: None,
        phone: None,
    };

    println!("Please enter employee name to search:");
    io::stdin()
        .read_line(&mut employee.name)
        .expect("You didn't enter an employee name");
    employee.name = employee.name.trim().to_string();

    let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();
    let employees: Vec<Employee> = coll
        .find(doc! {"name" : &employee.name}, find_options)
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
