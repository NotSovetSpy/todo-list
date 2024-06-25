use::console::Term;
use dialoguer::Select;
use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;
use std::{fs, env, path::PathBuf};

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
    let menu = vec!["1. Add a new task", "2. Mark a task as done", "3. Remove a task", "4. Exit"];
    let selection = Select::new()
        .with_prompt("Menu")
        .items(&menu)
        .interact()
        .unwrap();
    Ok((Status::Exit))
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