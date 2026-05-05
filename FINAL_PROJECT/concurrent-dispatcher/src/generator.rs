use crate::task::{Task, TaskKind};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub fn run_generator(tx: Sender<Task>, total: usize) {
    let mut rng = StdRng::seed_from_u64(42);

    for id in 0..total {
        let kind = if rng.gen_bool(0.7) {
            TaskKind::IO
        } else {
            TaskKind::CPU
        };

        let task = Task {
            id,
            arrival_time: Instant::now(),
            start_time: None,
            kind,
        };

        if tx.send(task).is_err() {
            break;
        }

        thread::sleep(Duration::from_millis(20));
    }
}