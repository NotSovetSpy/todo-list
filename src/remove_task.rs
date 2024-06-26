use crate::Status;

pub fn remove_task() -> Result<Status, std::io::Error> {
    Ok(Status::Running)
}