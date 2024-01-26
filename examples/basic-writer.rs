use rsv::writer::Writer;
use serde::Serialize;

#[derive(Serialize)]
struct ExampleStruct {
    a: i32,
    b: String,
    c: Option<String>,
    d: Option<u64>
}

fn main() {
    let mut w = Writer::from_writer(std::io::stdout());
    for i in 0..10 {
        let _ = w.serialize(ExampleStruct { a: 1, b: "".to_string(), c: Some(i.to_string()), d: None} );
    }
}
