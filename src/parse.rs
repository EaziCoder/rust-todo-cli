use crate::{
    DATA_FILE,
    todo::{Status, Storable, TodoList},
};

pub enum Command {
    Exit,
    Help,
    List,
    ListByStatus(Status),
    Add(String),
    Update(usize, String),
    Remove(usize),
    Clear,
    Save,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Command::Unknown(String::new());
    }

    match parts[0].to_lowercase().as_str() {
        "exit" | "quit" => Command::Exit,
        "help" => Command::Help,
        "list" | "ls" => {
            // Support: list, list todo, list done
            if parts.len() > 1 {
                if let Ok(status) = Status::from_str(parts[1]) {
                    return Command::ListByStatus(status);
                }
            }
            Command::List
        }
        "add" => {
            if parts.len() < 2 {
                println!("⚠️  Usage: add <task_description>");
                return Command::Unknown("add".to_string());
            }
            let description = parts[1..].join(" ");
            Command::Add(description)
        }
        "update" | "status" => {
            if parts.len() < 3 {
                println!("⚠️ Usage: update <task_number> <new_status>");
                return Command::Unknown("update".to_string());
            }
            match parts[1].parse::<usize>() {
                Ok(index) => Command::Update(index, parts[2].to_string()),
                Err(_) => {
                    println!("⚠️ Invalid task number.");
                    Command::Unknown("update".to_string())
                }
            }
        }
        "remove" | "delete" => {
            if parts.len() < 2 {
                println!("⚠️ Usage: remove <task_number>");
                return Command::Unknown("remove".to_string());
            }
            match parts[1].parse::<usize>() {
                Ok(index) => Command::Remove(index),
                Err(_) => {
                    println!("⚠️ Invalid task number.");
                    Command::Unknown("remove".to_string())
                }
            }
        }
        "clear" => Command::Clear,
        "save" => Command::Save,
        _ => Command::Unknown(input.to_string()),
    }
}

// ============================================================
// COMMAND HANDLERS - Clean separation of concerns
// ============================================================

pub fn handle_add(todo: &mut TodoList, description: String) {
    match todo.add_tasks(description) {
        Ok(_) => println!("✅ Task added successfully!"),
        Err(error) => println!("Error: {}", error),
    }
}

pub fn handle_update(todo: &mut TodoList, index: usize, status_str: &str) {
    match todo.update_task_status_str(index, status_str) {
        Ok(_) => println!("✅ Task status updated successfully!"),
        Err(error) => println!("Error: {}", error),
    }
}

pub fn handle_remove(todo: &mut TodoList, index: usize) {
    match todo.remove_task(index) {
        Ok(task) => println!("✅ Removed: {}", task.description),
        Err(error) => println!("Error: {}", error),
    }
}

pub fn handle_clear(todo: &mut TodoList) {
    let count = todo.clear_completed();
    if count > 0 {
        println!("🗑️  Cleared {} completed task(s)", count);
    } else {
        println!("⚠️  No completed tasks to clear");
    }
}

pub fn handle_save(todo: &TodoList) {
    match todo.save(DATA_FILE) {
        Ok(_) => println!(" Tasks saved to {}", DATA_FILE),
        Err(error) => println!("Failed to save: {}", error),
    }
}

pub fn list_tasks(todo: &TodoList, filter_status: Option<Status>) {
    let tasks = match filter_status {
        Some(status) => todo.filter_by_status(status),
        None => todo.list_tasks(),
    };

    if tasks.is_empty() {
        if filter_status.is_some() {
            println!("📝 No tasks with that status");
        } else {
            println!("📝 No tasks yet. Add one with: add <description>");
        }
        return;
    }

    println!("\n📋 Your Tasks:");
    println!("─────────────────────────────────────");
    for (index, task) in tasks {
        let icon = match task.status {
            Status::Todo => "⚪",
            Status::InProgress => "🔵",
            Status::Completed => "✅",
        };
        println!("{} {}. {}", icon, index, task);
    }
    println!("─────────────────────────────────────");
}

pub fn print_help() {
    println!("Commands:");
    println!("  add <description>        Add a new task");
    println!("  list [status]            List all tasks (or filter by status)");
    println!("  update <num> <status>    Update task status (todo/in-progress/done)");
    println!("  remove <num>             Remove a task");
    println!("  clear                    Remove all completed tasks");
    println!("  save                     Save tasks to file");
    println!("  help                     Show this help message");
    println!("  exit                     Save and exit");
    println!();
    println!("Examples:");
    println!("  add Buy groceries");
    println!("  list done");
    println!("  update 1 in-progress");
    println!("  remove 2");
}
