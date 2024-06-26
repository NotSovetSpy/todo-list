use crate::Status;

pub fn mark_task() -> Result<Status, std::io::Error> {
    Ok(Status::Running)
}

