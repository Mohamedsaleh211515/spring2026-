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
    pub start_time: Option<Instant>,
    pub kind: TaskKind,
}

impl Task {
    pub fn duration(&self) -> Duration {
        match self.kind {
            TaskKind::CPU => Duration::from_millis(350),
            TaskKind::IO => Duration::from_millis(100),
        }
    }

    pub fn wait_time(&self) -> Option<u128> {
        self.start_time.map(|start| {
            start.duration_since(self.arrival_time).as_millis()
        })
    }

    pub fn turnaround_time(&self) -> Option<u128> {
        self.start_time.map(|start| {
            start.elapsed().as_millis()
        })
    }
}