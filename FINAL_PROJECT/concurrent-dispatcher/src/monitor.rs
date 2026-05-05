use crate::state::SystemState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Snapshot {
    pub cpu: f64,
    pub workers: usize,
}

pub fn start_monitor(
    state: Arc<Mutex<SystemState>>,
    snapshots: Arc<Mutex<Vec<Snapshot>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(10));

            let s = state.lock().unwrap();

            snapshots.lock().unwrap().push(Snapshot {
                cpu: s.cpu_in_use,
                workers: s.active_workers,
            });

            if s.finished_tasks >= s.total_tasks {
                break;
            }
        }
    })
}