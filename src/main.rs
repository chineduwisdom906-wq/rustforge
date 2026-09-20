mod operation;

use operation::Operation;
use std::io;
use std::io::Write;

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    let id: u64 = read_line("Enter operation ID: ")
        .parse()
        .expect("Please enter a valid integer");

    let name = read_line("Enter operation name: ");

    let mut op = Operation::new(id, name);

    match op.start() {
        Ok(()) => println!("Operation started"),
        Err(e) => println!("{}", e),
    }

    match op.fail() {
        Ok(()) => println!("Operation failed"),
        Err(e) => println!("{}", e),
    }

    println!("Project: {}", op.name());
    println!("ID: {}", op.id());
    println!("Status: {}", op.status());
}
