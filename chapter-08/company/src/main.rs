use std::collections::{HashMap, HashSet};
use std::io;

// Commands:
// add [employee] to [department]
// remove [employee] from [department]
// create department [department]
// delete department [department]
// print department [department]
// print all
// exit

fn main() {
    let mut departments: HashMap<String, HashSet<String>> = HashMap::new();

    departments.insert(
        "department1".to_string(),
        HashSet::new()
    );

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => { 
                println!("Failed to read input!");
                panic!();
            }
        }

        let args: Vec<&str> = input.split_whitespace().collect();

        match args[0] {
            "add" => {
                if args.len() != 4 {
                    println!("ERROR: Incorrect number of arguments:");
                    println!("add [employee] to [department]");
                    continue
                }

                if args[2] != "to" {
                    println!("ERROR: Incorrect command syntax:");
                    println!("add [employee] to [department]");
                    continue
                }

                if !departments.contains_key(args[3]) {
                    println!(
                        "ERROR: It seems that department doesn't exist, please create it first!"
                    );
                    continue
                }

                departments
                    .entry(args[3].to_string())
                    .or_insert(HashSet::new())
                    .insert(args[1].to_string());

                println!("SUCCESS: Added employee '{}' to department '{}'", args[1], args[3]);
            },

            "remove" => {
                if args.len() != 4 {
                    println!("ERROR: Incorrect number of arguments:");
                    println!("remove [employee] from [department]");
                    continue
                }

                if args[2] != "from" {
                    println!("ERROR: Incorrect command syntax:");
                    println!("remove [employee] from [department]");
                    continue
                }

                if !departments.contains_key(args[3]) {
                    println!(
                        "ERROR: It seems that department doesn't exist, please create it first!"
                    );
                    continue
                }

                let entry = departments.entry(args[3].to_string())
                    .or_insert(HashSet::new())
                    .contains(args[1]);

                if !entry {
                    println!("ERROR: That employee is not in that department!");
                    continue
                }

                departments
                    .entry(args[3].to_string())
                    .or_insert(HashSet::new())
                    .remove(args[1]);

                println!("SUCCESS: Removed employee '{}' from department '{}'", args[1], args[3]);
            },

            "create" => {
                if args.len() != 3 {
                    println!("ERROR: Incorrect number of arguments:");
                    println!("create department [department]");
                    continue
                }

                if args[1] != "department" {
                    println!("ERROR: Incorrect syntax:");
                    println!("create department [department]");
                    continue
                }

                if departments.contains_key(args[2]) {
                    println!("ERROR: That department already exists!");
                    continue
                }

                departments.entry(args[2].to_string()).or_insert(HashSet::new());
                println!("SUCCESS: Created department: '{}'", args[2]);
            },

            "delete" => {
                if args.len() != 3 {
                    println!("ERROR: Incorrect number of arguments:");
                    println!("delete department [department]");
                    continue
                }

                if args[1] != "department" {
                    println!("ERROR: Incorrect syntax:");
                    println!("delete department [department]");
                    continue
                }

                if departments.contains_key(args[2]) {
                    println!("ERROR: That department does not exist!");
                    continue
                }

                departments.insert(args[2].to_string(), HashSet::new());
                println!("SUCCESS: Deleted department: '{}'", args[2]);
            },

            "print" => {
                match args[1] {
                    "all" => {
                        for (department, employees) in &departments {
                            println!("Department: {}", department);
                            let mut employees: Vec<&String> = employees.iter().collect();
                            employees.sort();
                            for employee in employees {
                                println!("{}", employee);
                            }
                        }
                    },

                    "department" => {
                        match departments.get(args[2]) {
                                Some(employees) => {
                                println!("Department: {}", args[2]);
                                let mut employees: Vec<&String> = employees.iter().collect();
                                employees.sort();
                                for employee in employees {
                                    println!("{}", employee);
                                }
                            },

                            None => {
                                println!("ERROR: That department does not exist!");
                                continue
                            },
                        }
                    },

                    _ => {
                        println!("Incorrect command syntax:");
                        println!("print all");
                        println!("print department [department]");
                        continue
                    }
                }
            },

            "exit" => {
                println!("Goodbye!");
                break
            },

            _ => {
                println!("Invalid command, please enter a valid command!");
            }
        }
    }
}
