use std::time::Instant;

pub struct SystemState {
    pub shutdown: bool,
    pub active_workers: usize,
    pub cpu_in_use: f64,
    pub finished_tasks: usize,
    pub total_tasks: usize,
    pub start_time: Instant,

    pub total_wait_time: u128,
}

impl SystemState {
    pub fn new(total_tasks: usize) -> Self {
        Self {
            shutdown: false,
            active_workers: 0,
            cpu_in_use: 0.0,
            finished_tasks: 0,
            total_tasks,
            start_time: Instant::now(),

            total_wait_time: 0,
        }
    }
}