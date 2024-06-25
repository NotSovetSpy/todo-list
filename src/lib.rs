use::console::Term;
use dialoguer::{Select, Confirm};
use std::{fs, env};

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Running,
    Exit,
}

pub const TITLE: &str = "To-Do list"; 
pub const SECTION: &str = "----------------------------";

pub fn display_content(terminal: Term) -> Result<(), std::io::Error>{
    terminal.write_line(TITLE)?;
    terminal.write_line(SECTION)?;

    let tasks = read_list();
    let mut index: i32 = 1;
    for task in tasks {
        terminal.write_line(&format!("{index}.{task}"))?;
        index += 1;
    }
    terminal.write_line(SECTION)?;
    terminal.write_line("")?;
    Ok(())
}

pub fn run(terminal: Term) -> Result<Status, std::io::Error>{
    let menu = vec!["1. Add a new task", "2. Mark/unmark task as done", "3. Remove a task", "4. Exit"];
    let selection = Select::new()
        .with_prompt("Menu")
        .items(&menu)
        .interact()
        .unwrap();
    match selection {
        0 => {
            add_new_task()
        },
        1 => {
            mark_task()
        },
        2 => {
            remove_task()
        },
        3 => {
            let confirm = Confirm::new()
                            .with_prompt("Are you shure you whant exit? All your progress was saved.")
                            .interact()
                            .unwrap();
            if confirm {
                Ok(Status::Exit)
            } else {
                Ok(Status::Running)
            }
        },
        _ => {
            print!("Error: Invalid input");
            Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
}

fn read_list() -> Vec<String> {
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

fn add_new_task() -> Result<Status, std::io::Error> {
    Ok(Status::Running)
}

fn mark_task() -> Result<Status, std::io::Error> {
    Ok(Status::Running)
}

fn remove_task() -> Result<Status, std::io::Error> {
    Ok(Status::Running)
}