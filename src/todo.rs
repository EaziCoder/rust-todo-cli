use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs;
use thiserror::Error;

pub trait Storable {
    fn save(&self, path: &str) -> Result<(), TodoError>;
    fn load(path: &str) -> Result<Self, TodoError>
    where
        Self: Sized;
}

// Improved error handling using thiserror
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Index must start from 1")]
    InvalidIndex,

    #[error("Status {0} not recognized. Use: todo, in-progress, done")]
    InvalidStatus(String),

    #[error("No task exists at that index {0}")]
    IndexOutOfBound(usize),

    #[error("Task description cannot be empty")]
    EmptyDescription,

    #[error("Failed to serialize tasks: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Failed to access file: {0}")]
    FileError(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Completed,
}

impl Display for Status {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(formatter, "TODO"),
            Status::InProgress => write!(formatter, "IN-PROGRESS"),
            Status::Completed => write!(formatter, "DONE"),
        }
    }
}

impl Status {
    // to parse a status from a string
    pub fn from_str(status_str: &str) -> Result<Self, TodoError> {
        match status_str.to_lowercase().as_str() {
            "todo" | "to-do" => Ok(Status::Todo),
            "done" | "completed" => Ok(Status::Completed),
            "in-progress" | "inprogress" => Ok(Status::InProgress),
            _ => Err(TodoError::InvalidStatus(status_str.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn new(description: String) -> Result<Self, TodoError> {
        if description.trim().is_empty() {
            return Err(TodoError::EmptyDescription);
        }
        Ok(Task {
            description: description.trim().to_string(),
            status: Status::Todo,
        })
    }

    // Task Helper Method
    pub fn is_completed(&self) -> bool {
        self.status == Status::Completed
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.description, self.status)
    }
}

// TodoList - Main data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    // Add a task - now uses Task::new for validation
    pub fn add_tasks(&mut self, description: String) -> Result<(), TodoError> {
        let task = Task::new(description)?;
        self.tasks.push(task);
        Ok(())
    }

    // Get number of tasks
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    // List all tasks with a functional appraoch
    pub fn list_tasks(&self) -> Vec<(usize, &Task)> {
        self.tasks
            .iter()
            .enumerate()
            .map(|(i, task)| (i + 1, task))
            .collect()
    }

    // Filter tasks by status
    pub fn filter_by_status(&self, status: Status) -> Vec<(usize, &Task)> {
        self.tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.status == status)
            .map(|(i, task)| (i + 1, task))
            .collect()
    }

    // Update task status with better error handling
    pub fn update_task_status(
        &mut self,
        index: usize,
        new_status: Status,
    ) -> Result<(), TodoError> {
        self.validate_index(index)?;
        self.tasks[index - 1].status = new_status;
        Ok(())
    }

    // supports user input like: status 2 done
    pub fn update_task_status_str(
        &mut self,
        index: usize,
        status_str: &str,
    ) -> Result<(), TodoError> {
        let new_status = Status::from_str(status_str)?;
        self.update_task_status(index, new_status)?;
        Ok(())
    }

    // Remove a task
    pub fn remove_task(&mut self, index: usize) -> Result<Task, TodoError> {
        self.validate_index(index)?;
        Ok(self.tasks.remove(index - 1))
    }

    // Clear all completed tasks
    pub fn clear_completed(&mut self) -> usize {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| !task.is_completed());
        original_len - self.tasks.len()
    }

    // Helper to validate index
    fn validate_index(&self, index: usize) -> Result<(), TodoError> {
        if index == 0 {
            return Err(TodoError::InvalidIndex);
        }
        if index > self.tasks.len() {
            return Err(TodoError::IndexOutOfBound(index));
        }
        Ok(())
    }
}

// IIMPLEMENT THE STORABLE TRAIT
impl Storable for TodoList {
    fn save(&self, path: &str) -> Result<(), TodoError> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load(path: &str) -> Result<Self, TodoError> {
        match fs::read_to_string(path) {
            Ok(json) => {
                let tasks = serde_json::from_str(&json)?;
                Ok(TodoList { tasks })
            }
            Err(error) => Err(TodoError::FileError(error)),
        }
    }
}
