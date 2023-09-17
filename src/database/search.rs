use crate::database::employee::Employee;
use clearscreen::clear;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client};
use regex::Regex;
use std::io;

pub async fn search_employees(client: &Client) -> Result<(), mongodb::error::Error> {
    clear().expect("Failed to clear the screen at delete employee");

    let db = &client.database("Employee");
    let coll = db.collection::<Employee>("employees");

    let mut search_pattern = String::new();

    println!("Please enter employee name to search:");
    io::stdin()
        .read_line(&mut search_pattern)
        .expect("You didn't enter an employee name");

    search_pattern = search_pattern.trim().to_string();
    let regex_pattern = Regex::new(&format!("{}", search_pattern)).unwrap();

    let query = doc! {"name" : {
    "$regex": regex_pattern.to_string(),    // take whatever they enter and search for those characters
    "$options": "i"}}; // case insenstive

    let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();

    // return result sorted by name asc
    let employees: Vec<Employee> = coll
        .find(query, find_options)
        .await
        .expect("Unable to get cursor")
        .try_collect()
        .await
        .expect("Unable to get items in cursor");

    if employees.len() > 0 {
        for e in employees.iter() {
            println!("Employee: {}, Title: {}", e.name, e.title);
        }
    } else {
        println!("No records found");
    }

    Ok(())
}
