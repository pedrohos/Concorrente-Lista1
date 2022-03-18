use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std_semaphore::Semaphore;

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
    
    // Semaphore to protect critical section
    let sem = Arc::new(Semaphore::new(1));
    // Sharable counter to keep track of the number of threads that have finished
    let count = Arc::new(RwLock::new(0));
    
    for i in 0..num_threads {
        let sem = Arc::clone(&sem);
        let count = Arc::clone(&count);

        thread::spawn(move || {
            
            // Initial sleep
            let sleep_time = rand::thread_rng().gen_range(0..5);
            thread::sleep(Duration::from_secs(sleep_time));
            println!("Thread {} slept for {}s.", &i, sleep_time);
            
            // Increments the shared counter, protected between semaphores
            {
                sem.acquire();
                let count_lock = &mut *(count.write().unwrap());
                *count_lock += 1;

                //println!("The value INSIDE is {}", count_lock);
                sem.release();
            }
        });
    }

    // Keeps the main thread a while loop until all threads have finished
    let mut exit_condition = false;
    while !exit_condition {
        let count_lock = *(count.read().unwrap());
        //println!("The value is {}", count_lock);
        sem.acquire();
        if count_lock == num_threads {
            exit_condition = true;
        }
        sem.release();
    }

    // By this time, it is guaranteed that all threads have finished
    println!("The number of threads is {}.", num_threads);
}
