use std::io::{self, Write};
use std::fs::{self, OpenOptions};

fn main() {
    let mut todo: String = String::new();
    let mut answer: String = String::new();

    loop {
        todo.clear();
        answer.clear();

        println!("Enter your todo: ");
        io::stdin().read_line(&mut todo).expect("Error");

        println!("Confirm? [y/n]");
        io::stdin().read_line(&mut answer).expect("Error");

        if answer.trim() == "showtodos" || todo.trim() == "showtodos" {
            let mut text = fs::read_to_string("db.txt").expect("Error");
            println!("\nTodos: {}", text.lines().count());
            println!("{}", &mut text);
        }
        
        if answer.trim() == "y" {
            let mut file: fs::File = OpenOptions::new()
                .create(true)
                .append(true)
                .open("db.txt")
                .expect("Error");

            writeln!(file, "{}", todo.trim()).expect("Error");
            println!("Succssesful");
        } else if answer.trim() == "n" {
            println!("Enter your todo: ");
            io::stdin().read_line(&mut todo).expect("Error");
        } else {
            println!("Please enter y or n: ");
            io::stdin().read_line(&mut answer).expect("Error");
        }
    }
}

