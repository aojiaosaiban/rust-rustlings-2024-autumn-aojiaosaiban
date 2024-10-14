// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings
// threads2.rs  
//  
// Building on the last exercise, we want all of the threads to complete their  
// work but this time the spawned threads need to be in charge of updating a  
// shared value: JobStatus.jobs_completed  
//  
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a  
// hint.  



use std::sync::{Arc, Mutex};  
use std::thread;  
use std::time::Duration;  

struct JobStatus {  
    jobs_completed: u32,  
}  

fn main() {  
    // Wrap JobStatus in a Mutex for interior mutability and synchronization  
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));  
    let mut handles = vec![];  

    for _ in 0..10 {  
        let status_shared = Arc::clone(&status);  
        let handle = thread::spawn(move || {  
            thread::sleep(Duration::from_millis(250));  
            // Lock the mutex before updating the shared value  
            let mut status = status_shared.lock().unwrap();  
            status.jobs_completed += 1; // Update the jobs_completed safely  
        });  
        handles.push(handle);  
    }  

    // Join threads and print the number of completed jobs  
    for handle in handles {  
        handle.join().unwrap();  
    }  

    // After all threads have finished, lock the mutex and print the count  
    let final_status = status.lock().unwrap(); // Lock the mutex again to read the value  
    println!("jobs completed {}", final_status.jobs_completed); // Print the final count  
}