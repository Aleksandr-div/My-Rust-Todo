use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() {
    let mut todo: String = String::new();
    let mut answer: String = String::new();
    let mut vec: Vec<String> = Vec::new();

    loop {
        todo.clear();
        answer.clear();

        println!("Choose option: ");
        println!("1. Add todo\n2. Delete todo\n3. Show my todos\n");

        io::stdin().read_line(&mut answer).expect("Error");

        if answer.trim() == "3" {
            if vec.is_empty() {
                println!("Thats nothing to do!\n");
                continue;
            }

            for (i, value) in vec.iter().enumerate() {
                println!("{}: {}\n", i + 1, value);
            }

        } else if answer.trim() == "2" {
            if vec.is_empty() {
                println!("Thats nothing to do!\n");
                continue;
            }

            for (i, value) in vec.iter().enumerate() {
                println!("\n{}. {}", i + 1, value);
            }

            answer.clear();
            println!("Enter todo to delete: \n");
            io::stdin().read_line(&mut answer).expect("Error");

            if answer.trim().parse::<usize>().unwrap_or(0) > 0 {
                vec.remove(answer.trim().parse::<usize>().expect("Error") - 1);
                
            } else {
                vec.remove(0);
            }

        } else if answer.trim() == "1" {
            println!("Enter todo: \n");
            io::stdin().read_line(&mut todo).expect("Error");
            vec.push(todo.clone().trim().to_string());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open("db.txt")
                .expect("Error");
            writeln!(file, "{}", todo.clone()).expect("Error");
            println!("Sucssesful!\n")
        }
    }
}
