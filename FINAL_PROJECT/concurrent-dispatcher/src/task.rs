use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum TaskKind {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub arrival_time: Instant,
    pub start_time: Option<Instant>, // wait time
    pub kind: TaskKind,
}

impl Task {
    // fixed execution time model
    pub fn duration(&self) -> Duration {
        Duration::from_millis(200)
    }

    // used by scheduler/metrics
    pub fn cpu_cost(&self) -> f64 {
        match self.kind {
            TaskKind::CPU => 0.35,
            TaskKind::IO => 0.10,
        }
    }

    pub fn wait_time(&self) -> Option<u128> {
        self.start_time.map(|start| {
            start.duration_since(self.arrival_time).as_millis()
        })
    }
}