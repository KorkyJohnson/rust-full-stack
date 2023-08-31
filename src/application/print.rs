use clearscreen::{self, clear};
use std::collections::HashMap;

pub fn print_employees(employee_database: &mut HashMap<String, String>) {                    
    clear().expect("Failed to clear the screen at display menu");

    if employee_database.len() > 0 {
        let mut employee_vector: Vec<_> = employee_database.iter().collect();
        employee_vector.sort();
        
        for (key, value) in employee_vector {
            println!("Employee: {}Department: {}", key, value)
        }
    } else {
        println!("No entries have been made");
        crate::application::menu::display_menu();
    }
}