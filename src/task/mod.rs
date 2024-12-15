use serde::Serialize;
use serde_json;
use serde_json::to_writer_pretty;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::Mutex;

use chrono::prelude::*;
use chrono::DateTime;
use serde::Deserialize;

static TASK_LIST: LazyLock<Mutex<Vec<Task>>> = LazyLock::new(|| {
    let file_path = "tasks.json";

    let tasks: Vec<Task> = fs::read_to_string(file_path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default();

    if tasks.is_empty() {
        let _ =
            File::create(file_path).and_then(|mut file| Ok(to_writer_pretty(&mut file, &tasks)));
    }

    Mutex::new(tasks)
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
            Self::save_tasks(&tasks);
            print_list(&tasks);
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
            Self::save_tasks(&tasks);
            print_list(&tasks);
        } else {
            println!("task with Id {} could not be found", task_id);
        }
    }

    pub fn delete(task_id: u32) {
        let mut tasks = Self::get_task_list();

        tasks.retain(|x| x.id != task_id);
        Self::save_tasks(&tasks);
        print_list(&tasks);
    }

    pub fn add(task_name: &str) {
        let current_time = Local::now();
        let mut tasks = Self::get_task_list();
        let id = tasks.iter_mut().last().map_or(0, |task| task.id) + 1;

        let new_task = Task {
            id,
            status: TaskStatus::Todo,
            description: String::from(task_name),
            created_at: current_time,
            updated_at: current_time,
        };

        tasks.push(new_task);
        Self::save_tasks(&tasks);

        print_list(&tasks);
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

        print_list(&tasks);
    }

    pub fn list_tasks(status: Option<String>) {
        let tasks = Self::get_task_list();
        println!("printing......");
        if let Some(status) = status {
            let task_status = TaskStatus::from_str(&status).unwrap();
            let matching_tasks: Vec<Task> = tasks
                .iter()
                .filter(|x| x.status == task_status)
                .cloned()
                .collect();
            print_list(&matching_tasks);
        } else {
            print_list(&tasks);
        }
    }

    fn save_tasks(tasks: &Vec<Task>) {
        let file_name = "tasks.json";

        if let Ok(file) = File::create(file_name) {
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &tasks).unwrap();
            writer.flush().unwrap();
        }
    }
}

fn print_list(tasks: &Vec<Task>) {
    for task in tasks.iter() {
        println!();
        println!("id: {}", task.id);
        println!("status: {}", task.status);
        println!("description: {}", task.description);
        println!("created_at: {}", task.created_at.format("%d/%m/%Y %H:%M"));
        println!("modified_at: {}", task.updated_at.format("%d/%m/%Y %H:%M"));
    }
}
