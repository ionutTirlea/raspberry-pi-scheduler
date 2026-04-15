mod config;
mod loader;
mod runner;
mod scheduler;
mod task;

fn main() {
    let tasks = loader::load_tasks();
    scheduler::start(&tasks);
}
