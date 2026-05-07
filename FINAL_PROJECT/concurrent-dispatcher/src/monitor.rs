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

            // -------------------------
            // Read shared state once
            // -------------------------
            let (workers, finished, total) = {
                let s = state.lock().unwrap();
                (s.active_workers, s.finished_tasks, s.total_tasks)
            };

            // -------------------------
            // Derived CPU usage (IMPORTANT FIX)
            // -------------------------
            let cpu_usage = workers as f64 / 8.0; // 8 worker threads

            // -------------------------
            // Store snapshot
            // -------------------------
            snapshots.lock().unwrap().push(Snapshot {
                cpu: cpu_usage,
                workers,
            });

            // -------------------------
            // Stop condition
            // -------------------------
            if finished >= total {
                break;
            }
        }
    })
}