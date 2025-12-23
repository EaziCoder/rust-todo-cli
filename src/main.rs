use std::io::{self, Write};

use crate::{
    parse::{
        Command, handle_add, handle_clear, handle_remove, handle_save, handle_update, list_tasks,
        parse_command, print_help,
    },
    todo::{Storable, TodoList},
};

mod todo;

mod parse;

const DATA_FILE: &str = "tasks.json";

fn main() {
    println!("Welcome to the Todo CLI!");
    // println!("Type commands like: add \"Buy groceries\"");
    println!("Type 'exit' to quit the application.");
    println!("💡 Type 'help' to see available commands");
    println!("-----------------------------------");

    // Load existing tasks using the Storable trait
    let mut todo = match TodoList::load(DATA_FILE) {
        Ok(list) => {
            if !list.is_empty() {
                println!("✅ Loaded {} from existing tasks", list.len());
            }
            list
        }
        Err(error) => {
            println!("⚠️  Could not load tasks: {}", error);
            TodoList::new()
        }
    };

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match parse_command(input) {
            Command::Exit => {
                if let Err(error) = todo.save(DATA_FILE) {
                    println!("⚠️  Failed to save tasks: {}", error);
                } else {
                    println!("✅ Tasks saved successfully!");
                }
                println!(" Goodbye!");
                break;
            }
            Command::Help => print_help(),
            Command::List => list_tasks(&todo, None),
            Command::ListByStatus(status) => list_tasks(&todo, Some(status)),
            Command::Add(description) => handle_add(&mut todo, description),
            Command::Update(index, status_str) => handle_update(&mut todo, index, &status_str),
            Command::Remove(index) => handle_remove(&mut todo, index),
            Command::Clear => handle_clear(&mut todo),
            Command::Save => handle_save(&todo),
            Command::Unknown(cmd) => {
                println!("❓ Unknown command: '{}'", cmd);
                println!("💡 Type 'help' to see available commands");
            }
        }
    }
}
