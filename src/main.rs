// application imports
use clearscreen::{self, clear};
use rust_full_stack::application::menu::display_menu;
use std::io;

// database imports
use rust_full_stack::database::{
    add::insert_document,
    admin::{list_collections, list_databases},
    connect::check_database,
    print::print_database,
    delete::delete_employee,
};
use tokio;

#[tokio::main]
async fn main() {
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
                    if let Err(err) = print_database(&client).await {
                        eprint!("Error while printing records {}", err)
                    }
                }
                'd' => {
                    if let Err(err) = delete_employee(&client).await {
                        eprint!("Error while deleting an employee {}", err)
                    }
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
