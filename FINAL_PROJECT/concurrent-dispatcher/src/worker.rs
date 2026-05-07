use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::collections::VecDeque;

use crate::state::SystemState;
use crate::task::{Task, TaskKind};

pub fn start_worker(
    id: usize,
    queue: Arc<Mutex<VecDeque<Task>>>,
    state: Arc<Mutex<SystemState>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {

            // -------------------------
            // Fetch task
            // -------------------------
            let task = {
                let mut q = queue.lock().unwrap();
                q.pop_front()
            };

            // -------------------------
            // Exit condition
            // -------------------------
            if task.is_none() {
                let s = state.lock().unwrap();
                if s.finished_tasks >= s.total_tasks {
                    break;
                }
                drop(s);
                thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            let mut task = task.unwrap();

            task.start_time = Some(Instant::now());

            // -------------------------
            // TRACE OUTPUT (WHAT YOU WANTED)
            // -------------------------
            println!(
                "Worker {} → Task {} ({:?}) START",
                id,
                task.id,
                task.kind
            );

            // mark active
            {
                let mut s = state.lock().unwrap();
                s.active_workers += 1;
            }

            // simulate work
            thread::sleep(task.duration());

            // mark inactive
            {
                let mut s = state.lock().unwrap();
                s.active_workers -= 1;
            }

            println!(
                "Worker {} → Task {} ({:?}) FINISH",
                id,
                task.id,
                task.kind
            );

            // -------------------------
            // Update state
            // -------------------------
            let mut s = state.lock().unwrap();

            if let Some(wait) = task.wait_time() {
                let wait = wait as u128;

                s.total_wait_time += wait;

                if wait > s.max_wait_time {
                    s.max_wait_time = wait;
                }

                match task.kind {
                    TaskKind::IO => s.total_io_wait_time += wait,
                    TaskKind::CPU => s.total_cpu_wait_time += wait,
                }
            }

            if let Some(turn) = task.turnaround_time() {
                s.total_turnaround_time += turn;
            }

            match task.kind {
                TaskKind::CPU => s.cpu_completed += 1,
                TaskKind::IO => s.io_completed += 1,
            }

            s.finished_tasks += 1;
        }
    })
}