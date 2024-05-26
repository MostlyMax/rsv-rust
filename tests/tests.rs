use rsv_core::reader;
use rsv_core::utils::{NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE};
use std::collections::HashSet;

#[test]
fn seq_deserialization() {
    //type Rsv = Vec<HashSet<String>>;

    // empty buffer
    let buffer: Vec<u8> = vec![];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 0);
        row_count += 1;
    }
    assert!(row_count == 0);

    // 3 empty rows
    let buffer: Vec<u8> = vec![ROW_TERM_BYTE, ROW_TERM_BYTE, ROW_TERM_BYTE];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 0);
        row_count += 1;
    }
    assert!(row_count == 3);

    // 3 rows with empty strings
    let buffer: Vec<u8> = vec![VALUE_TERM_BYTE, ROW_TERM_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 0);
        row_count += 1;
    }
    println!("{:#?}", row_count);
    assert!(row_count == 3);

    // 2 rows with null values
    let buffer: Vec<u8> = vec![NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE, NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        println!("{:#?}", result);
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 1);
        row_count += 1;
    }
    println!("{:#?}", row_count);
    assert!(row_count == 2);

    // 5 rows with non-empty strings
    let buffer: Vec<u8> = vec![b'a', VALUE_TERM_BYTE, ROW_TERM_BYTE, b'a', b'b', b'c', VALUE_TERM_BYTE, ROW_TERM_BYTE, b'x', VALUE_TERM_BYTE, ROW_TERM_BYTE, b'y', VALUE_TERM_BYTE, ROW_TERM_BYTE, b'z', VALUE_TERM_BYTE, ROW_TERM_BYTE];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        println!("{:#?}", result);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.len() > 0);
        println!("{:#?}", value);
        row_count += 1;
    }
    println!("{:#?}", row_count);
    assert!(row_count == 5);
}