// application imports
use clearscreen::{self, clear};
use rust_full_stack::application::{add::add_employee, menu::display_menu, print::print_employees};
use std::{collections::HashMap, io};

// database imports
use rust_full_stack::mongodb::connect::{check_database, list_databases};
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
                    add_employee(&mut employee_database);
                }
                'p' => {
                    print_employees(&mut employee_database);
                }
                'm' => {
                    clear().expect("Failed to clear the screen at display menu");
                    display_menu();
                }
                'l' => { // hidden menu for admin purposes
                    clear().expect("Failed to clear the screen at list databases");
                    let _ = list_databases(&client).await;
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
