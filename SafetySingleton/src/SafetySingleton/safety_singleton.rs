use std::sync::{Mutex, Arc};
use std::thread;


pub struct SafetySingleton {
    pub data: i32,
}

lazy_static! {
    static ref INSTANCE: Mutex<SafetySingleton> = Mutex::new(SafetySingleton { data: 0 });
}

impl SafetySingleton {
    pub fn get_instance() -> &'static Mutex<SafetySingleton> {
        &INSTANCE
    }
}



