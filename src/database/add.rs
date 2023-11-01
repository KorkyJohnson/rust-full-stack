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

    // let mut name = String::new();
    // loop {
        println!("Please enter employee name to add:");
        // match io::stdin().read_line(&mut name) {
        //     Ok(name) => name,
        //     Err(err) => eprintln!("Please enter a name. {}", err),
        // }
        io::stdin()
            .read_line(&mut employee.name)
            .expect("You didn't enter a name");
        employee.name = employee.name.trim().to_string();

        // if !employee.name.is_empty() && is_chars_only(&employee.name) {
        //     break; // Exit the loop if a non-empty name is provided
        // } else {
        //     println!("Name cannot be empty and only contain alphabetic letters. Please enter a valid name.");
        // }
    // }

    // let mut result:String = String::new();

    // &mut employee.name = match io::stdin().read_line(&mut result) {
    //     Ok(name) => name,
    //     Err(error) => eprintln!("Please enter a valid name"),
    // };
    // loop {
    //     let mut is_valid_name: bool = true;

    //     match get_employee_name(&mut employee.name) {
    //         Ok(_) => {
    //             let name = employee.name.trim();
    //             // is_chars_only_result = is_chars_only(&name);
    //             // if !name.is_empty() && is_chars_only_result {
    //             if !name.is_empty() && is_valid_name && is_chars_only(&name) {
    //                 employee.name = name.to_string();
    //                 break;
    //             } else {
    //                 is_valid_name = false;
    //                 println!("Name cannnot be empty, please enter a valid value");
    //             }
    //         }
    //         Err(err) => {
    //             eprintln!("Error {}", err);
    //         }
    //     }
    // }

    println!("Please enter employee title");
    io::stdin()
        .read_line(&mut employee.title)
        .expect("You didn't enter an employee title");
    employee.title = employee.title.trim().to_string();

    println!("Please enter employee salary");
    io::stdin()
        .read_line(&mut employee.title)
        .expect("No salary entered");
    employee.title = employee.title.trim().to_string();

    let doc = vec![employee];

    coll.insert_many(doc, None).await?;
    println!("Record inserted");

    Ok(())
}

// fn get_employee_name(name: &mut String) -> Result<(), String> {
//     println!("Please enter an employee name");
//     io::stdin()
//         .read_line(name)
//         .map_err(|e| format!("Failed to read input: {}", e))?;
//     Ok(())
// }

fn is_chars_only(name: &str) -> bool {
    !name.chars().any(|c| c.is_numeric())
}
