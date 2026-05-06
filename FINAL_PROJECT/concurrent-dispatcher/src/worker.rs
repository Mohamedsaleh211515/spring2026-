use crate::state::SystemState;
use crate::task::Task;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_worker(
    id: usize,
    queue: Arc<Mutex<VecDeque<Task>>>,
    state: Arc<Mutex<SystemState>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            // Try to fetch a task (lock kept very short)
            let task = {
                let mut q = queue.lock().unwrap();
                q.pop_front()
            };

            match task {
                Some(mut task) => {
                    let start = std::time::Instant::now();
                    task.start_time = Some(start);

                    // Update active worker state
                    {
                        let mut s = state.lock().unwrap();
                        s.active_workers += 1;
                        s.cpu_in_use = (s.cpu_in_use + task.cpu_cost()).min(1.0);
                    }

                    println!("Worker {} running task {}", id, task.id);

                    // Simulate work
                    thread::sleep(task.duration());

                    // Update completion state
                    let mut s = state.lock().unwrap();

                    if let Some(wait) = task.wait_time() {
                        s.total_wait_time += wait;
                    }

                    s.finished_tasks += 1;
                    s.active_workers -= 1;
                    s.cpu_in_use = (s.cpu_in_use - task.cpu_cost()).max(0.0);

                    // Exit condition
                    if s.finished_tasks >= s.total_tasks {
                        break;
                    }
                }

                None => {
                    // Check if we're done
                    let done = {
                        let s = state.lock().unwrap();
                        s.finished_tasks >= s.total_tasks
                    };

                    if done {
                        break;
                    }

                    // Avoid busy waiting
                    thread::sleep(Duration::from_millis(5));
                }
            }
        }
    })
}