// application imports
use clearscreen::{self, clear};
use rust_full_stack::{
    application::{menu::display_menu, print::print_employees},
    database::connect::list_collections,
};
use std::{collections::HashMap, io};

// database imports
use rust_full_stack::database::{connect::check_database, connect::list_databases, add_employee::insert_document};
use tokio;

#[tokio::main]
async fn main() {
    let mut employee_database: HashMap<String, String> = HashMap::new();
    clear().expect("Failed to clear the screen at main");
    display_menu();

    let client = match check_database().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error while checking database: {}", err);
            return;
        }
    };

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("You didn't enter anything");

        if let Some(user_choice) = user_input.trim().chars().next() {
            match user_choice.to_ascii_lowercase() {
                'a' => {
                    if let Err(err) = insert_document(&client).await {
                        eprint!("Error while inserting a record {}", err)
                    }
                }
                'p' => {
                    print_employees(&mut employee_database);
                }
                'm' => {
                    clear().expect("Failed to clear the screen at display menu");
                    display_menu();
                }
                // hidden menu for admin purposes
                'l' => {
                    clear().expect("Failed to clear the screen at list databases");
                    let _ = list_databases(&client).await;
                }
                'c' => {
                    let _db_name = "Employee";
                    let _db = client.database(_db_name); //default db
                    clear().expect("Failed to clear the screen at list collections");
                    let _ = list_collections(&_db).await;
                }

                'q' => {
                    println!("Goodbye :)");
                    break;
                }
                _ => println!("Please make a valid selection, press 'M' to display the menu"),
            }
        } else {
            println!("No valid input entered");
        }
    }
}
