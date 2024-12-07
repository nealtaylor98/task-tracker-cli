use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::Mutex;

use chrono::prelude::*;
use chrono::DateTime;

static TASK_LIST: LazyLock<Mutex<Vec<Task>>> = LazyLock::new(|| Mutex::new(Vec::new()));

enum TaskStatus {
    InProgress,
    Done,
    Todo,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "In Progress"),
            Self::Done => write!(f, "Done"),
            _ => write!(f, "To do"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in-progress" => Ok(TaskStatus::InProgress),
            "todo" => Ok(TaskStatus::Todo),
            "done" => Ok(TaskStatus::Done),
            _ => Err(()),
        }
    }
}

pub struct Task {
    id: u32,
    status: TaskStatus,
    description: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    fn get_task_list() -> std::sync::MutexGuard<'static, Vec<Task>> {
        TASK_LIST
            .lock()
            .map_err(|_| "Failed to acquire lock")
            .unwrap()
    }

    pub fn mark_done(task_id: u32) {
        let current_time = Local::now();
        let mut tasks = Self::get_task_list();

        if let Some(task) = tasks.iter_mut().find(|x| x.id == task_id) {
            task.updated_at = current_time;
            task.status = TaskStatus::Done;
        } else {
            println!("task with Id {} could not be found", task_id);
        }
    }

    pub fn mark_in_progress(task_id: u32) {
        let current_time = Local::now();
        let mut tasks = Self::get_task_list();

        if let Some(task) = tasks.iter_mut().find(|x| x.id == task_id) {
            task.updated_at = current_time;
            task.status = TaskStatus::InProgress;
        } else {
            println!("task with Id {} could not be found", task_id);
        }
    }

    pub fn delete(task_id: u32) {
        let mut tasks = Self::get_task_list();

        tasks.retain(|x| x.id != task_id);
    }

    pub fn add(task_name: &str) {
        let current_time = Local::now();
        let mut tasks = Self::get_task_list();

        let new_task = Task {
            id: 1,
            status: TaskStatus::Todo,
            description: String::from(task_name),
            created_at: current_time,
            updated_at: current_time,
        };
        tasks.push(new_task);

        println!("adding task");
        print_list(tasks);
    }

    pub fn update(task_id: u32, task_name: &str) {
        let current_time = Local::now();

        let mut tasks = Self::get_task_list();

        if let Some(task) = tasks.iter_mut().find(|x| x.id == task_id) {
            task.description = task_name.to_string();
            task.updated_at = current_time;
        } else {
            println!("task with Id {} could not be found", task_id);
        }
    }

    pub fn list_tasks(status: Option<String>) {
        let tasks = Self::get_task_list();
        println!("printing......");
        if let Some(status) = status {
            print_list(tasks);
            let task_status = TaskStatus::from_str(&status);
        } else {
            print_list(tasks);
        }
    }
}

fn print_list(tasks: std::sync::MutexGuard<'_, Vec<Task>>) {
    for task in tasks.iter() {
        println!();
        println!("id: {}", task.id);
        println!("status: {}", task.status);
        println!("description: {}", task.description);
        println!("created_at: {}", task.created_at);
        println!("modified_at: {}", task.updated_at);
    }
}
