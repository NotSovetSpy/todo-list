use crate::Status;
use dialoguer::{Select, Confirm};
use crate::task_list_operation::write_to_list;


pub fn remove_task(tasks: &mut Vec<String>) -> Result<Status, std::io::Error> {
    let select = Select::new()
                .with_prompt("Select task to remove")
                .items(&tasks)
                .default(0)
                .interact()
                .unwrap();
    let task = tasks.get(select).unwrap();
    let confirm = Confirm::new()
                .with_prompt(&format!("Are you shure you whant to remove task: '{task}'?"))
                .interact()
                .unwrap();
    if confirm {
        tasks.remove(select);
        return write_to_list(tasks, None)
    }
    Ok(Status::Running)
}