use::console::Term;
use dialoguer::{Select, Confirm};

pub mod create_task;
pub mod mark_task;
pub mod remove_task;
pub mod task_list_operation;

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Running,
    Exit,
}

pub const TITLE: &str = "To-Do list"; 
pub const SECTION: &str = "----------------------------";
pub const MENU: [&str; 4]= ["1. Add a new task", "2. Mark/unmark task as done", "3. Remove a task", "4. Exit"];


pub fn display_content(terminal: Term, tasks: &Vec<String>) -> Result<(), std::io::Error>{
    terminal.write_line(TITLE)?;
    terminal.write_line(SECTION)?;

    let mut index: i32 = 1;
    for task in tasks {
        terminal.write_line(&format!("{index}.{task}"))?;
        index += 1;
    }
    terminal.write_line(SECTION)?;
    terminal.write_line("")?;
    Ok(())
}

pub fn run(terminal: Term, tasks: &mut Vec<String>) -> Result<Status, std::io::Error>{
    let selection = Select::new()
        .with_prompt("Menu")
        .items(&MENU)
        .interact()
        .unwrap();
    match selection {
        0 => {
            create_task::add_new_task(tasks)
        },
        1 => {
            mark_task::mark_task(tasks)
        },
        2 => {
            remove_task::remove_task(tasks)
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

