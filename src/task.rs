use std::str::FromStr;

use cron::Schedule;

pub struct Task {
    pub name: String,
    pub command: String,
    pub schedule: Schedule,
}

impl Task {
    pub fn new(name: String, command: String, schedule_expr: &str) -> Task {
        let schedule = Schedule::from_str(schedule_expr).expect("invalid schedule expression");

        Task {
            name,
            command,
            schedule,
        }
    }
}
