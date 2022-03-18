use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    // Get number of threads from user
    let num_threads: u32 = loop {
        println!("Please type the number of threads you want to instantiate:");

        let mut num_threads = String::new();

        io::stdin()
            .read_line(&mut num_threads)
            .expect("Error: Failed reading input.");
        
        match num_threads.trim().parse() {
            Ok(num) => {
                if num <= 0 {
                    println!("Error: The input \"number of threads\" must be >= 1.\n");
                    continue;
                }
                break num;
            },

            Err(_) => {
                println!("Error: Failed while converting input to non-negative int.\n");
                continue;
            }
        };
    };
    
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    // Spawns threads and pushes their handles to the vector above
    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            // Put every thread to sleep for a random amount of time
            let sleep_time = rand::thread_rng().gen_range(0..5);
            thread::sleep(Duration::from_secs(sleep_time));
            println!("Thread {} slept for {}s.", &i, sleep_time);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    // PS: The threads are already joined by the main thread by default, so this joining above is not necessary.
    // Although for educational purposes, it does make the idea crystal clear.

    println!("The number of threads is {}.", num_threads);
}
