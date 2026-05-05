use crate::state::SystemState;
use crate::task::{Task, TaskKind};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc::Receiver};
use std::thread;
use std::time::Duration;

pub fn run_dispatcher(
    rx: Receiver<Task>,
    queue: Arc<Mutex<VecDeque<Task>>>,
    state: Arc<Mutex<SystemState>>,
) {
    let mut cpu_queue = VecDeque::new();
    let mut io_queue = VecDeque::new();

    loop {
        if let Ok(task) = rx.try_recv() {
            match task.kind {
                TaskKind::CPU => cpu_queue.push_back(task),
                TaskKind::IO => io_queue.push_back(task),
            }
        }

        {
            let s = state.lock().unwrap();
            if s.finished_tasks >= s.total_tasks {
                break;
            }
        }

        let next = if !io_queue.is_empty() {
            io_queue.pop_front()
        } else {
            cpu_queue.pop_front()
        };

        if let Some(task) = next {
            queue.lock().unwrap().push_back(task);
        }

        thread::sleep(Duration::from_millis(1));
    }
}