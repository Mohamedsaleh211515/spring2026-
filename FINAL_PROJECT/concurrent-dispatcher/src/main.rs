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

fn print_row(label: &str, value: impl std::fmt::Display) {
    println!("{:<30}: {}", label, value);
}

fn main() {
    let total_tasks = 1000;

   
    let (tx, rx) = mpsc::channel::<Task>();

    // Shared queue
    let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));

    // Global system state
    let state = Arc::new(Mutex::new(SystemState::new(total_tasks)));

    // Monitoring snapshots
    let snapshots = Arc::new(Mutex::new(Vec::<Snapshot>::new()));


    let gen_handle = {
        let tx = tx.clone();
        thread::spawn(move || run_generator(tx, total_tasks))
    };

    // Dispatcher
   
    let dispatch_handle = {
        let queue = Arc::clone(&queue);
        let state = Arc::clone(&state);
        thread::spawn(move || run_dispatcher(rx, queue, state))
    };

    
    let mut workers = vec![];

    for id in 0..8 {
        workers.push(start_worker(
            id,
            Arc::clone(&queue),
            Arc::clone(&state),
        ));
    }

    drop(tx); // close generator channel

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

    let finished = s.finished_tasks.max(1) as f64;

    let avg_wait =
        s.total_wait_time as f64 / finished;

    let avg_turnaround =
        s.total_turnaround_time as f64 / finished;

    // IO / CPU split (REQUIRED FOR OPTIMIZED)
    let avg_wait_io =
        s.total_io_wait_time as f64 / s.io_completed.max(1) as f64;

    let avg_wait_cpu =
        s.total_cpu_wait_time as f64 / s.cpu_completed.max(1) as f64;

    let total_runtime = s.start_time.elapsed();

  
    println!("\n====================");
    println!("== FIFO simulation ==");
    println!("====================");

    let io = s.io_completed;
    let cpu = s.cpu_completed;
    let total = io + cpu;

    let io_pct = if total > 0 { (io as f64 / total as f64) * 100.0 } else { 0.0 };
    let cpu_pct = if total > 0 { (cpu as f64 / total as f64) * 100.0 } else { 0.0 };

    let workers = 8;
    let cap = 100;

    println!(
        "{} tasks , ({:.1}%) IO / ({:.1}%) CPU , {} workers , Cap: {}",
        total,
        io_pct,
        cpu_pct,
        workers,
        cap
    );

    print_row("total runtime", format!("{} ms", total_runtime.as_millis()));
    print_row("makespan", format!("{} ms", total_runtime.as_millis()));

    print_row(
        "tasks completed",
        format!(
            "{} (IO={}, CPU={})",
            s.finished_tasks, s.io_completed, s.cpu_completed
        )
    );

    print_row("avg wait time", format!("{:.2} ms", avg_wait));
    print_row("avg turnaround time", format!("{:.2} ms", avg_turnaround));
    print_row("max wait time", format!("{} ms", s.max_wait_time));

    print_row("avg CPU usage", format!("{:.2}%", avg_cpu * 100.0));
    print_row("avg workers active", format!("{:.2} / 8", avg_workers));

    print_row("monitor samples", snaps.len());
    print_row("monitor csv", "monitor_log.csv");

    
    println!("\n==============================");
    println!("== Optimized simulation ==");
    println!("==============================");

    let io = s.io_completed;
    let cpu = s.cpu_completed;
    let total = io + cpu;

    let io_pct = if total > 0 { (io as f64 / total as f64) * 100.0 } else { 0.0 };
    let cpu_pct = if total > 0 { (cpu as f64 / total as f64) * 100.0 } else { 0.0 };

    let workers = 8;
    let cap = 100;

    println!(
        "{} tasks , ({:.1}%) IO / ({:.1}%) CPU , {} workers , Cap: {}",
        total,
        io_pct,
        cpu_pct,
        workers,
        
        cap
    );

    print_row("total runtime", format!("{} ms", total_runtime.as_millis()));
    print_row("makespan", format!("{} ms", total_runtime.as_millis()));

    print_row(
        "tasks completed",
        format!(
            "{} (IO={}, CPU={})",
            s.finished_tasks, s.io_completed, s.cpu_completed
        )
    );

    print_row("avg wait time", format!("{:.2} ms", avg_wait));
    print_row("avg wait (IO only)", format!("{:.2} ms", avg_wait_io));
    print_row("avg wait (CPU only)", format!("{:.2} ms", avg_wait_cpu));

    print_row("avg turnaround time", format!("{:.2} ms", avg_turnaround));
    print_row("max wait time", format!("{} ms", s.max_wait_time));

    print_row("avg CPU usage", format!("{:.2}%", avg_cpu * 100.0));
    print_row("avg workers active", format!("{:.2} / 8", avg_workers));

    print_row("monitor samples", snaps.len());
    print_row("monitor csv", "monitor_log.csv");
}