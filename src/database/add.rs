use crate::database::employee::{self, Employee};
use clearscreen::clear;
use mongodb::Client;
use std::{io, thread::AccessError};

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

    // let mut name = String::new();
    // println!("Please enter employee name to add:");
    // match io::stdin().read_line(&mut name) {
    //     Ok(name) => name,
    //     Err(err) => eprintln!("Please enter a name. {}", err),
    // }
    // io::stdin()
    // .read_line(&mut employee.name)
    // .expect("Please enter a name");
    // employee.name = employee.name.trim().to_string();

    // let mut result:String = String::new();

    // &mut employee.name = match io::stdin().read_line(&mut result) {
    //     Ok(name) => name,
    //     Err(error) => eprintln!("Please enter a valid name"),
    // };
    loop {
        match get_employee_name(&mut employee.name) {
            Ok(_) => {
                let name = employee.name.trim();
                if !name.is_empty() {
                    employee.name = name.to_string();
                    break;
                } else {
                    println!("Name cannnot be empty, please enter a valid value");
                }
            }
            Err(err) => {
                eprintln!("Error {}", err);
            }
        }
    }

    fn get_employee_name(name: &mut String) -> Result<(), String> {
        println!("Please enter an employee name");
        io::stdin()
            .read_line(name)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        Ok(())
    }

    println!("Please enter employee salary");
    io::stdin()
        .read_line(&mut employee.title)
        .expect("No salary entered");
    employee.title = employee.title.trim().to_string();

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
