use std::{thread, time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SomeStruct {
    some_field: i64,
}

fn main() {
    for _ in 0..42 {
        println!("Hello, world!");
    }
    thread::sleep(time::Duration::from_secs(60));
}
