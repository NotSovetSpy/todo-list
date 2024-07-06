use crate::Status;
use dialoguer::Select;
use crate::task_list_operation::write_to_list;

pub fn mark_task(tasks: &mut Vec<String>) -> Result<Status, std::io::Error> {
    let select = Select::new()
        .with_prompt("Select task")
        .items(tasks)
        .interact()
        .unwrap();
    let task = tasks.get(select).unwrap().clone();
    if is_striked(task.clone()) {
        tasks[select] = remove_strike(task);
    } else {
        tasks[select] = strike_through(task);
    }
    write_to_list(tasks, None)
}

const START_CODE: &str = "\x1B[9m";
const END_CODE: &str = "\x1B[0m";

fn strike_through(text: String) -> String{
    format!("{}{}{}", START_CODE, text, END_CODE)
}

fn is_striked(text: String) -> bool {
    text.starts_with(START_CODE) && text.ends_with(END_CODE)
}

fn remove_strike(text: String) -> String {
    text.trim_start_matches(START_CODE).trim_end_matches(END_CODE).to_string()
}