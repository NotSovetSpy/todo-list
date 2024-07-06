use crate::Status;
use dialoguer::Input;
use crate::task_list_operation::write_to_list;

pub fn add_new_task(tasks: &mut Vec<String>) -> Result<Status, std::io::Error> {
    let task: String = Input::new()
                .with_prompt("New task")
                .interact()
                .unwrap();
    write_to_list(tasks, Some(task))
}