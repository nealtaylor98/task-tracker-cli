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

pub struct Task {
    id: u32,
    status: TaskStatus,
    description: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    pub fn mark_done(task_id: &str) {
        todo!()
    }

    pub fn mark_in_progress(task_id: &str) {
        todo!()
    }

    pub fn delete(task_id: u32) {
        todo!()
    }

    pub fn add(task_name: &str) {
        let mut tasks = TASK_LIST
            .lock()
            .map_err(|_| "Failed to acquire lock")
            .unwrap();
        let current_time = Local::now();
        let new_task = Task {
            id: 1,
            status: TaskStatus::Todo,
            description: String::from(task_name),
            created_at: current_time,
            updated_at: current_time,
        };
        tasks.push(new_task);
    }

    pub fn update(task_id: u32, task_name: &str) {
        todo!()
    }

    pub(crate) fn list_tasks(status: Option<String>) {
        todo!()
    }
}
