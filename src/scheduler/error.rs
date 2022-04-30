use std::{error, fmt};

pub type SchedulerResult<T> = Result<T, SchedulerError>;

#[derive(Debug, Clone)]
pub enum SchedulerError {
  DuplicateTaskId(String),
  TaskNotFound(String),
}

impl fmt::Display for SchedulerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SchedulerError::DuplicateTaskId(id) => write!(f, "a task already exists with the id {}", id),
      SchedulerError::TaskNotFound(id) => write!(f, "could not find a task with the id {}", id),
    }
  }
}

impl error::Error for SchedulerError {}
