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
        println!("1. Add todo\n2. Delete todo\n3. Show m
y todos\n");

        io::stdin().read_line(&mut answer).expect("Error
");

        if answer.trim() == "3" {
            if vec.is_empty() {
                println!("Thats nothing to do!\n");
                continue;
            }

            for (i, value) in vec.iter().enumerate() {
                println!("{}. {}", i + 1, value);
            }

        } else if answer.trim() == "2" {
            if vec.is_empty() {
                println!("Thats nothing to do!\n");
                continue;
            }

            for (i, value) in vec.iter().enumerate() {
                println!("{}. {}", i + 1, value);
            }

            answer.clear();
            println!("Enter todo to delete: \n");
            io::stdin().read_line(&mut answer).expect("E
rror");
            let mut parsed = answer.trim().parse::<usize
>().unwrap_or(0);

            if vec.is_empty() {
                println!("\nThats nothing to do!");
                continue;
            }  

            if parsed > 0 && parsed <= vec.len() {
                vec.remove(parsed - 1);

            } else {
                println!("\nTodo doesn't exists!");
            }

        } else if answer.trim() == "1" {
            println!("Enter todo: \n");
            io::stdin().read_line(&mut todo).expect("Err
or");
            vec.push(todo.clone().trim().to_string());

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("db.txt")
                .expect("Error");
            writeln!(file, "{}", todo.clone().trim()).ex
pect("Error");
            println!("Sucssesful!\n");
        }
    }
}