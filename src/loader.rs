use crate::config::Config;
use crate::task::Task;

pub fn load_tasks() -> Vec<Task> {
    let config = Config::load().expect("failed to load tasks.toml");

    let mut tasks = Vec::new();

    for configured_task in config.tasks {
        let task = Task::new(
            configured_task.name,
            configured_task.command,
            &configured_task.schedule,
        );
        println!("Loaded task '{}'", task.name);
        tasks.push(task);
    }

    tasks
}
