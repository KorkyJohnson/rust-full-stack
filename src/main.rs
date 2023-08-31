use clearscreen::{self, clear};
use std::{io, collections::HashMap};
use rust_full_stack::application::{menu::display_menu, print::print_employees, add::add_employee};

fn main() {

    let mut employee_database: HashMap<String, String> = HashMap::new();
    clear().expect("Failed to clear the screen at main");
    display_menu();

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

