use std::sync::{RwLock, Arc};

pub struct SafetySingleton {
    pub data: i32,
}

lazy_static! {
    static ref INSTANCE: Arc<RwLock<SafetySingleton>> = Arc::new(RwLock::new(SafetySingleton { data: 0 }));
}

impl SafetySingleton {
    pub fn get_instance() -> &'static Arc<RwLock<SafetySingleton>> {
        &INSTANCE
    }

    pub fn Update(&mut self) {
        println!("Call Update  ! !");
        self.data += 1;
    }
}
