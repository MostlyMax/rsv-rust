use std::fs::File;

use rsv_core::reader::Reader;
use rsv_core::writer::Writer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ExampleStruct {
    _num: i32,
    _string: String,
    _option: Option<f64>,
    // _vec_option: Vec<Option<f64>>,
}

// ~~~~~~~~~~~
// Add the zstd crate in order to run this example!!
// ~~~~~~~~~~~

fn writer() {
    // We are able to write RSV objects into anything that implements the write trait.
    // this includes things like files, stdio, and compressors.
    let f = File::create("zstd-example.bin").unwrap();
    let w = zstd::stream::write::Encoder::new(f, 0).unwrap();
    let w = w.auto_finish();

    // Since the zstd encoder is already internally buffered, we can create
    // our RSV encoder with no buffering.
    let mut w = Writer::from_writer_unbuffered(w);

    let a = ExampleStruct { _num: 30202, _string: "Hello Stenway!".to_string(), _option: None };
    let b = ExampleStruct { _num: -30202, _string: "Hello Stenway!".to_string(), _option: Some(3.14) };

    w.serialize(&a).unwrap();
    w.serialize(&b).unwrap();
}

fn reader() {
    // We are able to read RSV objects from anything that implements the read trait.
    // this includes things like files, stdio, and compressors.
    let f = File::open("zstd-example.bin").unwrap();
    let r = zstd::stream::read::Decoder::new(f).unwrap();

    let mut r = Reader::from_reader(r);
    let mut r = r.deserialize::<ExampleStruct>();

    println!("{:#?}", r.next());
    println!("{:#?}", r.next());
    assert!(r.next().is_none());
}

fn main() {
    writer();
    reader();
}
