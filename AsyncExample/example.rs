#[macro_use]
extern crate lazy_static;
use tokio::time::{sleep, Duration};
use tokio::task;
use std::borrow::{Borrow, BorrowMut};
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
        self.data += 1;
        // self.PrintData();
    }

    pub fn PrintData(&mut self) {
        println!("Data : {:?}", self.data);
    }

    pub fn GetData(&self) -> i32 {
        return self.data.clone()
    }
}


async fn async_func1(loop_cnt: i32) {
    let mut cnt = 0;

    loop {
        if SafetySingleton::get_instance().write().unwrap().GetData() >= loop_cnt {
            println!("================================= Async Function 1 End ! ! ! ! ! =================================");
            break;
        }

        // println!("async function 1 : {:?}", cnt);
        // cnt += 1;

        SafetySingleton::get_instance().write().unwrap().Update();
        println!("Update by Async Func 1 : {:?}", SafetySingleton::get_instance().read().unwrap().GetData());

        sleep(Duration::from_millis(1000));
    }
}

async fn async_func2(loop_cnt: i32) {
    let mut cnt = 0;

    loop {
        if SafetySingleton::get_instance().write().unwrap().GetData() >= loop_cnt {
            println!("================================= Async Function 2 End ! ! ! ! ! =================================");
            break;
        }

        // println!("async function 2 : {:?}", cnt);
        // cnt += 1;


        SafetySingleton::get_instance().write().unwrap().Update();
        println!("Update by Async Func 2 : {:?}", SafetySingleton::get_instance().read().unwrap().GetData());
        sleep(Duration::from_millis(1000));
    }
}

async fn async_func3(loop_cnt: i32) {
    let mut cnt = 0;

    loop {
        if SafetySingleton::get_instance().write().unwrap().GetData() >= loop_cnt {
            println!("================================= Async Function 3 End ! ! ! ! ! =================================");
            break;
        }

        // println!("async function 3 : {:?}", cnt);
        // cnt += 1;

        SafetySingleton::get_instance().write().unwrap().Update();
        println!("Update by Async Func 3 : {:?}", SafetySingleton::get_instance().read().unwrap().GetData());
        sleep(Duration::from_millis(1000));
    }
}

#[tokio::main]
async fn main() {
    let f1 = task::spawn(async_func1(10000));
    let f2 = task::spawn(async_func2(1000));
    let f3 = task::spawn(async_func3(2600));

    let _ = tokio::join!(f1, f2, f3);
}
