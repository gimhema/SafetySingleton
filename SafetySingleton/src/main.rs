#[macro_use]
extern crate lazy_static;

mod SafetySingleton;
use SafetySingleton::{*, example::test_concurrency};



fn main() {
    test_concurrency();
}
