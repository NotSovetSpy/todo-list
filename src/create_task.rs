use crate::Status;
use dialoguer::Input;

pub fn add_new_task() -> Result<Status, std::io::Error> {
    let task: String = Input::new()
                .with_prompt("New task")
                .interact()
                .unwrap();
    Ok(Status::Running)
}