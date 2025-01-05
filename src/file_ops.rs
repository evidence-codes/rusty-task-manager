use crate::task::Task;
use std::fs;

pub fn save_tasks_to_file(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(tasks)?;
    fs::write("tasks.json", data)?;
    println!("Tasks saved!");
    Ok(())
}

pub fn load_tasks_from_file() -> Result<Vec<Task>, std::io::Error> {
    let data = fs::read_to_string("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    println!("Tasks loaded!");
    Ok(tasks)
}
