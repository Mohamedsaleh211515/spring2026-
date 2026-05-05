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
            let task = {
                let mut q = queue.lock().unwrap();
                q.pop_front()
            };

            match task {
                Some(task) => {
                    println!("Worker {} running task {}", id, task.id);

                    thread::sleep(task.duration());

                    let mut s = state.lock().unwrap();
                    s.finished_tasks += 1;
                }

                None => {
                    thread::sleep(Duration::from_millis(5));
                }
            }
        }
    })
}