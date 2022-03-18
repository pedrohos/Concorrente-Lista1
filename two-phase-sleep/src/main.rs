use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc, RwLock};
use std_semaphore::Semaphore;

fn main() {
    // Get number of threads from user for phase 1
    let num_threads : usize = loop {
        let mut num_threads = String::new();

        println!("Please input the \"number of threads\" to be spawned.\n");
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

    let mutex = Arc::new(Mutex::new(0));
    let sem = Arc::new(Semaphore::new(0));
    let mut handles = Vec::new(); 
    let mut sleep_times = Arc::new(RwLock::new(vec![0; num_threads]));
    
    for i in 0..num_threads {
        let mutex = Arc::clone(&mutex);
        let sleep_time = rand::thread_rng().gen_range(0..6);
        let sem = Arc::clone(&sem);
        let sleep_times = Arc::clone(&mut sleep_times);
        
        // Spawn threads
        let handle = thread::spawn(move || {
            
            // Sleep for sleep_time seconds and generate the s sleep time.
            thread::sleep(Duration::from_secs(sleep_time));
            println!("Thread {} slept for {}s during phase one.", &i, sleep_time);

            let s = rand::thread_rng().gen_range(0..10);

            // Store the s sleep time on the proper next position of sleep_times vector
            {
                let sleep_times = &mut *(sleep_times.write().unwrap());
                sleep_times[(i + 1) % num_threads] = s;
            }
            
            //println!("Thread {} is going for position {}", &i, (i + 1) % num_threads);
            //println!("Going for barrier!");
            
            // Prepare the barrier and start phase 2
            {
                let mut val = mutex.lock().unwrap();
                *val += 1;
                drop(val);
            }

            {
                if *(mutex.lock().unwrap()) == num_threads {
                    sem.release();
                }
            }
            sem.acquire();
            sem.release();

            // Get thread's sleep time
            let sleep_times = &*(sleep_times.read().unwrap());
            let this_s = sleep_times[i].clone();
            drop(sleep_times);
            
            thread::sleep(Duration::from_secs(this_s));
            println!("Thread {} slept for {}s during phase two.", &i, this_s);
        });

        handles.push(handle);
    }

    // Just like explanied in the "fork-sleep-join" project, this is not necessary.
    for handle in handles {
        handle.join().unwrap();
    }

}
