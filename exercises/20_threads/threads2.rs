// by [apalala@gmail.com](https://github.com/apalala)
// by Gemini (2026-01-28)

use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // We wrap the struct in a Mutex to allow thread-safe mutation
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the mutex to get a mutable reference to the data inside.
            // The lock is automatically released when `status_lock` goes out of scope.
            status_shared.lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // We must lock the mutex one last time to read the final value.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
