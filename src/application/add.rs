use std::collections::HashMap;
use std::io;
use clearscreen::{self, clear};

pub fn add_employee(employee_database: &mut HashMap<String, String>) -> HashMap<String, String> {
    let mut emp_name = String::new();
    let mut emp_dept = String::new();
    clear().expect("Failed to clear the screen at add employee");

    println!("Please enter employee name:");
    io::stdin()
        .read_line(&mut emp_name)
        .expect("You didn't enter an employee name");

    println!("Please enter employee department");
    io::stdin()
        .read_line(&mut emp_dept)
        .expect("You didn't enter an employee department");

    employee_database.insert(emp_name, emp_dept);
    println!("Employee added");
    
    employee_database.clone()
}
