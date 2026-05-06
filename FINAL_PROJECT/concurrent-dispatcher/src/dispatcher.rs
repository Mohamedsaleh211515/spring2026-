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
    let mut cpu_queue: VecDeque<Task> = VecDeque::new();
    let mut io_queue: VecDeque<Task> = VecDeque::new();

    // used for round-robin fairness between IO and CPU
    let mut pick_io = false;

    loop {
        // 1. Pull incoming tasks (non-blocking)
        while let Ok(task) = rx.try_recv() {
            match task.kind {
                TaskKind::CPU => cpu_queue.push_back(task),
                TaskKind::IO => io_queue.push_back(task),
            }
        }

        // 2. Check if all tasks are completed
        {
            let state_guard = state.lock().unwrap();
            if state_guard.finished_tasks >= state_guard.total_tasks {
                break;
            }
        }

        // 3. Select next task fairly
        let next_task = match (!cpu_queue.is_empty(), !io_queue.is_empty()) {
            (true, true) => {
                pick_io = !pick_io;

                if pick_io {
                    io_queue.pop_front()
                } else {
                    cpu_queue.pop_front()
                }
            }
            (true, false) => cpu_queue.pop_front(),
            (false, true) => io_queue.pop_front(),
            (false, false) => None,
        };

        // 4. Dispatch task to shared execution queue
        if let Some(task) = next_task {
            if let Ok(mut q) = queue.lock() {
                q.push_back(task);
            }
        }

        // 5. Prevent busy spinning
        thread::sleep(Duration::from_millis(1));
    }
}