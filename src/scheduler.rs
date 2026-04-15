use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::runner::run_tasks;
use crate::task::Task;

pub fn start(tasks: &[Task]) {
    println!("scheduler started");

    loop {
        let due = find_due_tasks(tasks);

        if due.is_empty() {
            println!("no tasks to run");
            return;
        }

        sleep_until(due[0].1);
        run_tasks(&due);
    }
}

fn find_due_tasks(tasks: &[Task]) -> Vec<(&Task, DateTime<Utc>)> {
    let mut earliest_time: Option<DateTime<Utc>> = None;

    for task in tasks {
        let next = task.schedule.upcoming(Utc).next();

        if next.is_none() {
            continue;
        }

        let next = next.unwrap();

        if earliest_time.is_none() || next < earliest_time.unwrap() {
            earliest_time = Some(next);
        }
    }

    if earliest_time.is_none() {
        return Vec::new();
    }

    let earliest_time = earliest_time.unwrap();
    let mut due = Vec::new();

    for task in tasks {
        let next = task.schedule.upcoming(Utc).next();

        if next.is_none() {
            continue;
        }

        if next.unwrap() == earliest_time {
            due.push((task, earliest_time));
        }
    }

    due
}

fn sleep_until(time: DateTime<Utc>) {
    let now = Utc::now();
    let difference = time - now;
    let milliseconds = difference.num_milliseconds();

    if milliseconds > 0 {
        thread::sleep(Duration::from_millis(milliseconds as u64));
    }
}
