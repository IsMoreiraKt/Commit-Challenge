use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

const THREAD_COUNT: usize = 8;

fn main() {
    let commit_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..THREAD_COUNT {
        let commit_count = Arc::clone(&commit_count);
        
        let handle = thread::spawn(move || {
            loop {
                let mut count = commit_count.lock().unwrap();
                let commit_message = format!("Commit number {}", *count);
                let commit_status = Command::new("git")
                    .arg("commit")
                    .arg("--allow-empty")
                    .arg("-m")
                    .arg(&commit_message)
                    .status()
                    .expect("Failed to execute git commit");

                if !commit_status.success() {
                    eprintln!("git commit failed.");
                    return;
                }

                *count += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}