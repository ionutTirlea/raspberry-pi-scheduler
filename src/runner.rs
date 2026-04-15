use std::process::Command;

use chrono::{DateTime, Utc};

use crate::task::Task;

pub fn run_tasks(due: &[(&Task, DateTime<Utc>)]) {
    for (task, _) in due {
        println!("Task '{}' started", task.name);

        let result = Command::new(&task.command).spawn();

        if result.is_err() {
            eprintln!("Task '{}' failed: {}", task.name, result.unwrap_err());
        }
    }
}
