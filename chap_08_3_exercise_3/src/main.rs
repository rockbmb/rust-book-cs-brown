use std::{io, collections::HashMap};

type Company = HashMap<String, Vec<String>>;

fn main() {
    println!("Options:");
    println!("1. Add {{employee}} to {{department}}");
    println!("2. Retrieve employes from {{department}}");
    println!("3. List all employees in the company, sorted ascendingly.");
    println!("4. Quit");

    let mut company : Company = HashMap::new();
    loop {
        println!("Please input your request.");

        let mut switch = String::new();

        io::stdin()
        .read_line(&mut switch)
        .expect("Failed to read line");

        let switch : u32 = match switch.trim().parse() {
            Ok(num) => num,
            Err(_)       => continue
        };

        match switch {
            1 => adder(&mut company),
            2 => list_department(&mut company),
            3 => list_all_employees(&mut company),
            4 => {
                println!("Goodbye!");
                return
            }
            _ => {
                println!("Try again!");
                continue
            }
        }
    }

}

/// Add an employee to a department. Handles input from the user,
/// and creates a new entry in the hashmap if there was none before.
fn adder(company : &mut Company) {
    println!("Please input your submission.");
    let mut request = String::new();
    io::stdin()
    .read_line(&mut request)
    .expect("Failed to read line");

    let words = request
    .split_whitespace()
    .map(|str| str.to_string())
    .collect::<Vec<String>>();

    if words.len() != 4 || (words[0] == "Add" && words[1] == "to") {
        println!("Improper input! try again");
        return
    }

    let dep_employees = company
        .entry(words[3].clone())
        .or_insert(Vec::new());
    dep_employees.push(words[1].clone());
}

fn list_department(company : &mut Company) {
    println!("Please input the department you wish to list.");
    let mut request = String::new();
    io::stdin()
    .read_line(&mut request)
    .expect("Failed to read line");

    let department = request.trim().to_string();

    match company.get(&department) {
        None => println!("The selected department does not exist!"),
        Some(empls) => {
            println!("The employees in the selected department are:");
            println!("{:?}", empls);
        }
    }
}

fn list_all_employees(company : &mut Company) {
    let mut all_emps = Vec::new();
    for dep_emps in company.values() {
        for dep_emp in dep_emps.iter() {
            all_emps.push(dep_emp.clone());
        }
    }

    all_emps.sort();
    println!("The company's employees, sorted, are:");
    println!("{:?}", all_emps);
}