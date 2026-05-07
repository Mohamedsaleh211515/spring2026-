use std::time::Instant;

pub struct SystemState {
    pub active_workers: usize,

    pub finished_tasks: usize,
    pub total_tasks: usize,

    pub start_time: Instant,

    // metrics
    pub total_wait_time: u128,
    pub max_wait_time: u128,

    pub total_turnaround_time: u128,

    pub io_completed: usize,
    pub cpu_completed: usize,

    pub total_io_wait_time: u128,
    pub total_cpu_wait_time: u128,
}

impl SystemState {
    pub fn new(total_tasks: usize) -> Self {
        Self {
            active_workers: 0,

            finished_tasks: 0,
            total_tasks,

            start_time: Instant::now(),

            total_wait_time: 0,
            max_wait_time: 0,
            total_io_wait_time: 0,
            total_cpu_wait_time: 0,

            total_turnaround_time: 0,

            io_completed: 0,
            cpu_completed: 0,
        }
    }
}