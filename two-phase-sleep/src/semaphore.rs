// use std::sync::{Condvar, Mutex};

// pub struct Semaphore {
//     mutex: Mutex<u32>,
//     condvar: Condvar,
// }

// pub impl Semaphore {
//     pub fn new (count: u16) -> Self {
//         Semaphore {
//             mutex: Mutex::new(count as u32),
//             condvar: Condvar::new(),
//         }
//     }

//     pub fn dec (&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock -= 1;

//         while *lock < 0 {
//             lock = self.condvar.wait(lock).unwrap();
//         }
//     }

//     pub fn inc (&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock += 1;
//         self.condvar.notify_one();
//     }
// }