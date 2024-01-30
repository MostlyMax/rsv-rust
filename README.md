# rsv
an rsv reader and writer crate for Rust.

### what is rsv?
rows of string values (rsv) is a modification on the csv format that replaces
delimiter characters with unused unicode bytes. This makes encoding and decoding
incredibly simple and consistent.

find the specification created by Stenway [here](https://github.com/Stenway/RSV-Specification)

### example

```rust
use rsv::reader::Reader;
use rsv::writer::Writer;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct ExampleStruct {
    _num: i32,
    _string: String,
    _option: Option<f64>,
    // _vec_option: Vec<Option<f64>>,
}


fn writer() {
    let mut w = Writer::from_path("basic-serde-example.bin").unwrap();
    let a = ExampleStruct { _num: 30202, _string: "Hello Stenway!".to_string(), _option: None };
    let b = ExampleStruct { _num: -30202, _string: "Hello Stenway!".to_string(), _option: Some(3.14) };


    w.serialize(&a).unwrap();
    w.serialize(&b).unwrap();
}


fn reader() {
    let mut r = Reader::from_path("basic-serde-example.bin").unwrap();


    // If the compiler is unable to implicitly determine the type you may need
    // to explicitly set it.
    let mut r = r.deserialize::<ExampleStruct>();


    println!("{:#?}", r.next().unwrap());
    println!("{:#?}", r.next().unwrap());
    assert!(r.next().is_none());
}
```
