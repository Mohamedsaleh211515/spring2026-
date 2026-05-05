mod task;
mod generator;
mod dispatcher;
mod worker;
mod state;
mod monitor;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::VecDeque;

use generator::run_generator;
use dispatcher::run_dispatcher;
use worker::start_worker;
use monitor::{start_monitor, Snapshot};
use state::SystemState;
use task::Task;

fn main() {
    let total_tasks = 50;

    // channel: generator → dispatcher
    let (tx, rx) = mpsc::channel();

    // shared queue for workers
    let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));

    // global state
    let state = Arc::new(Mutex::new(SystemState::new(total_tasks)));

    // monitor snapshots
    let snapshots = Arc::new(Mutex::new(Vec::<Snapshot>::new()));

    // -------------------------
    // Generator thread
    // -------------------------
    let gen_handle = {
        let tx = tx.clone();
        thread::spawn(move || run_generator(tx, total_tasks))
    };

    // -------------------------
    // Dispatcher thread
    // -------------------------
    let dispatch_handle = {
        let queue = Arc::clone(&queue);
        let state = Arc::clone(&state);
        thread::spawn(move || run_dispatcher(rx, queue, state))
    };

   
    let mut workers = vec![];

    for i in 0..8 {
        workers.push(start_worker(
            i,
            Arc::clone(&queue),
            Arc::clone(&state),
        ));
    }

    // close generator channel
    drop(tx);

   
    let monitor_handle = {
        let state = Arc::clone(&state);
        let snaps = Arc::clone(&snapshots);
        start_monitor(state, snaps)
    };

    gen_handle.join().unwrap();
    dispatch_handle.join().unwrap();

    for w in workers {
        w.join().unwrap();
    }

    monitor_handle.join().unwrap();

   
    let snaps = snapshots.lock().unwrap();
    let len = snaps.len().max(1) as f64;

    let avg_cpu: f64 =
        snaps.iter().map(|s| s.cpu).sum::<f64>() / len;

    let avg_workers: f64 =
        snaps.iter().map(|s| s.workers as f64).sum::<f64>() / len;

    let s = state.lock().unwrap();

    let avg_wait =
        s.total_wait_time as f64 / s.finished_tasks.max(1) as f64;

 
    println!("\n===== RESULTS =====");
    println!("Tasks completed: {}", s.finished_tasks);
    println!("Average CPU usage: {:.2}%", avg_cpu * 100.0);
    println!("Average workers active: {:.2}", avg_workers);
    println!("Average wait time: {:.2} ms", avg_wait);
    println!("Total execution time: {:.2?}", s.start_time.elapsed());
}