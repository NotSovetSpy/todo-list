use crate::Status;
use std::{fs, env};

pub fn read_list() -> Vec<String> {
    let binding = env::current_dir()
                                    .unwrap()
                                    .join("src\\todo_list.txt");
    let path = binding
                                    .to_str()
                                    .unwrap();
    let data = fs::read_to_string(path).expect("Enable read file!");
    
    data
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn write_to_list(tasks: &mut Vec<String>, task: Option<String>) -> Result<Status, std::io::Error> {
    let binding = env::current_dir()
                                    .unwrap()
                                    .join("src\\todo_list.txt");
    let path = binding
                                    .to_str()
                                    .unwrap();
    if task.is_some() {
        tasks.push(task.unwrap());

    }
    let tasks = tasks.join("\n");
    fs::write(path, tasks)?;
    Ok(Status::Running)    
}