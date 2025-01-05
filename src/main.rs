mod command;
mod file_ops;
mod task;

use command::Command;
use file_ops::{load_tasks_from_file, save_tasks_to_file};
use std::io;
use task::Task;

fn process_command(command: Command, tasks: &mut Vec<Task>, id_counter: &mut usize) {
    match command {
        Command::Add(title) => {
            tasks.push(Task {
                id: *id_counter,
                title,
                completed: false,
            });
            *id_counter += 1;
            println!("Task added!");
        }
        Command::View => {
            for task in tasks {
                println!(
                    "[{}] {} - {}",
                    task.id,
                    task.title,
                    if task.completed {
                        "Completed"
                    } else {
                        "Incomplete"
                    }
                );
            }
        }
        Command::Complete(id) => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                println!("Task marked as completed!");
            } else {
                println!("Task not found!");
            }
        }
        Command::Delete(id) => {
            if tasks.iter().any(|t| t.id == id) {
                tasks.retain(|t| t.id != id);
                println!("Task deleted!");
            } else {
                println!("Task not found!");
            }
        }
        Command::Save => save_tasks_to_file(tasks).unwrap(),
        Command::Load => *tasks = load_tasks_from_file().unwrap_or_default(),
        Command::Exit => {
            println!("Goodbye!");
            std::process::exit(0);
        }
    }
}

fn get_user_command() -> Command {
    println!("Choose an action: Add, View, Complete, Delete, Save, Load, Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "Add" => {
            println!("Enter task title:");
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            Command::Add(title.trim().to_string())
        }
        "View" => Command::View,
        "Complete" => {
            println!("Enter task ID to mark as completed:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            Command::Complete(id.trim().parse().unwrap())
        }
        "Delete" => {
            println!("Enter task ID to delete:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            Command::Delete(id.trim().parse().unwrap())
        }
        "Save" => Command::Save,
        "Load" => Command::Load,
        "Exit" => Command::Exit,
        _ => {
            println!("Invalid command!");
            get_user_command()
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut id_counter = 1;

    loop {
        let command = get_user_command();
        process_command(command, &mut tasks, &mut id_counter);
    }
}
