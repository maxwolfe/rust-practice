use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq)]
enum Department {
    Sales,
    Marketing,
    Business,
    Engineering,
}

fn main() {
    let mut employees: Vec<String> = Vec::new();
    let mut departments: HashMap<String, Department> = HashMap::new();

    loop {
        let mut input = String::new();

        println!("Please Enter a command:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match find_nth_word(&input[..], 1) {
            "Add" => add_employee(&input, &mut employees, &mut departments),
            "List" => list_department(&input, &employees, &departments),
            "Sorted" => sort_all_employees(&mut employees, &departments),
            "End" => break,
            _ => println!("Command Invalid, please try again!"),
        };

        println!();
    }
}

fn find_nth_word(sentence: &str, mut n: i32) -> &str {
    let mut start = 0;
    let mut end = sentence.trim().len();

    for (idx, letter) in sentence.trim().as_bytes().iter().enumerate() {
        match letter {
            b' ' => {
                if n > 1 {
                    n -= 1;
                    start = idx + 1;
                } else {
                    end = idx;
                    break;
                }
            }
            _ => (),
        };
    }
    &sentence.trim()[start..end]
}

fn add_employee(
    input: &String,
    employees: &mut Vec<String>,
    departments: &mut HashMap<String, Department>,
) {
    let mut name = String::new();
    name.push_str(find_nth_word(&input[..], 2));

    let department = match find_nth_word(&input[..], 4) {
        "Sales" => Department::Sales,
        "Marketing" => Department::Marketing,
        "Business" => Department::Business,
        "Engineering" => Department::Engineering,
        _ => {
            println!("Department Invalid, please try again!");
            return;
        }
    };

    departments.insert(name.clone(), department);
    employees.push(name);
}

fn list_department(
    input: &String,
    employees: &Vec<String>,
    departments: &HashMap<String, Department>,
) {
    let department = match find_nth_word(&input[..], 2) {
        "Sales" => Department::Sales,
        "Marketing" => Department::Marketing,
        "Business" => Department::Business,
        "Engineering" => Department::Engineering,
        _ => {
            println!("Department Invalid, Please Try again!");
            return;
        }
    };

    println!("Listing all Employees in Department: {:?}", department);

    for employee in employees.iter() {
        match departments.get(employee) {
            Some(dep) => {
                if department == *dep {
                    println!("{}: {:?}", employee, department);
                }
            }
            _ => continue,
        };
    }
}

fn sort_all_employees(employees: &mut Vec<String>, departments: &HashMap<String, Department>) {
    employees.sort();

    let list_of_departments = [
        Department::Business,
        Department::Engineering,
        Department::Marketing,
        Department::Sales,
    ];

    for department in list_of_departments.iter() {
        println!("Listing all Employees in Department: {:?}", department);

        for employee in employees.iter() {
            match departments.get(employee) {
                Some(dep) => {
                    if *department == *dep {
                        println!("{}: {:?}", employee, department);
                    }
                }
                _ => continue,
            };
        }

        println!();
    }
}
