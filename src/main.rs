use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
    a: u32,
    b: String,
}

fn main() {
    let t = Test {
        a: 4,
        b: "Four".to_string(),
    };
}
