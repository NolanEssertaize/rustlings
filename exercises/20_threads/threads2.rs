// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_done: Mutex<u32>, // Use Mutex to allow mutable access to jobs_done
}

fn main() {
    let status = Arc::new(JobStatus { jobs_done: Mutex::new(0) }); // Initialize Mutex

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the mutex to safely update the shared value
            let mut jobs_done = status_shared.jobs_done.lock().unwrap();
            *jobs_done += 1; // Increment the jobs_done count
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the value of JobStatus.jobs_done.
    let jobs_done = status.jobs_done.lock().unwrap(); // Lock the mutex to read the value
    println!("Jobs done: {}", *jobs_done); // Dereference to get the value
}

