use core::task;

use::console::Term;
use dialoguer::Select;

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Running,
    Exit,
}

pub const TITLE: &str = "To-Do list application"; 

pub fn display_content(terminal: Term) -> Result<(), std::io::Error>{
    let tasks: Vec<&str> = vec!["Task 1", "Task 2", "Task 3"];
    let mut index = 1;
    for task in tasks {
        terminal.write_line(&format!("{index}.{task}"))?; 
    }
    terminal.write_line("")?;
    Ok(())
}

pub fn run(terminal: Term) -> Result<Status, std::io::Error>{
    let menu = vec!["1. Add a new task", "2. Mark a task as done", "3. Remove a task", "4. Exit"];
    let selection = Select::new()
        .with_prompt("Menu")
        .items(&menu)
        .interact()
        .unwrap();
    Ok((Status::Exit))
}
