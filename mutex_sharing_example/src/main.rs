use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // creating a variable to an i32
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // spawn 10 threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // ith acquisition of lock
            let mut num = counter.lock().unwrap();

            *num += 1;

        // subsequent release of lock
        });
        handles.push(handle);
    }

    for handle in handles {
        // ensure that all handles have finished
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
