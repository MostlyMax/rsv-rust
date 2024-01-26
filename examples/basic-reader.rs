use rsv::reader::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExampleStruct {
    _a: i32,
    _b: String,
    _c: Option<String>,
    _d: Option<u64>
}

fn main() {
    let mut w = Reader::from_path("test").unwrap();
    for rec in w.deserialize::<ExampleStruct>() {
        let s = rec.unwrap();
        println!("{:#?}", s);
    }
}
