use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use super::safety_singleton::*;

fn test_concurrency() {

    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            loop {

                let mut singleton_lock = SafetySingleton::get_instance().lock().unwrap();


                println!("Thread {}: Data: {}", i, singleton_lock.data);


                singleton_lock.data += 1;


                thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }

    // 모든 스레드의 종료를 기다립니다.
    for handle in handles {
        handle.join().unwrap();
    }
}