use std::io;
use std::collections::HashMap;

fn main() {
    // Create a HashMap to map each department to a list of employees 
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    // Why use Vec<String> as the value?
    // because each dept can have multiple people and vectors are perfect for holding lists of
    // things that grow
    
    println!("Enter name and department: ");
    println!("Type exit to stop.");
   
    loop
    {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Try again.");

        let input = input.trim();   // remove trailing newline, borrows from original String and
                                    // returns a &str

        if input == "exit"
        {
            break;
        }

        // Split user input into parts, creates a vector of string slices
        let parts: Vec<&str> = input
            .split_whitespace()
            .collect();

        // Error handling to make sure user inputs name and dept
        if parts.len() != 2 
        {
            println!("Please put name and department: ");
            continue;
        }

        // Converts parts(&str) to a String and puts it into `name`
        let name = parts[0].to_string();
        let department = parts[1].to_string();

        // Add an employee name to corresponding department
        company
            .entry(department)  // looks up key in hashmap
            .or_insert(Vec::new()) // if dept doesn't exit yet, make one
            .push(name);
       
        // Collect all department names into a vector of references to the keys in the
        // hashmap(company) to be able to sort them
        let mut departments: Vec<&String> = company
            .keys()
            .collect();

        departments.sort();

        println!("----Company Directory----");
        
        for x in departments
        {
            let employees = company[x]
                .join(", ");
            println!("{}: {}", x, employees);
        }
    }
}
