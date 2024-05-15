use std::sync::{RwLock, Arc};
use std::thread;
use std::time::Duration;
use crate::safety_singleton::*; // assuming safety_singleton is in a module called crate

pub fn test_concurrency() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            loop {
                {
                    let singleton_lock = SafetySingleton::get_instance().read().unwrap();
                    println!("Thread {}: Data: {}", i, singleton_lock.data);
                }

                {
                    let mut singleton_lock = SafetySingleton::get_instance().write().unwrap();
                    // singleton_lock.data += 1;
                    singleton_lock.Update();
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}
