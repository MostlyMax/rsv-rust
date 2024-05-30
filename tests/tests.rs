use rsv_core::reader;
use rsv_core::utils::{NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum NoValue {
    Empty {},
}

#[test]
fn seq_deserialization() {
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
    let buffer: Vec<u8> = vec![
        VALUE_TERM_BYTE, ROW_TERM_BYTE,
        VALUE_TERM_BYTE, ROW_TERM_BYTE,
        VALUE_TERM_BYTE, ROW_TERM_BYTE
    ];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        let row = result.unwrap();
        assert!(row.len() == 1);
        assert!(row[0] == "");
        row_count += 1;
    }
    assert!(row_count == 3);

    // 5 rows with non-empty strings
    let buffer: Vec<u8> = vec![
        b'a', VALUE_TERM_BYTE, ROW_TERM_BYTE,
        b'a', b'b', b'c', VALUE_TERM_BYTE, ROW_TERM_BYTE,
        b'x', VALUE_TERM_BYTE, ROW_TERM_BYTE,
        b'y', VALUE_TERM_BYTE, ROW_TERM_BYTE,
        b'z', VALUE_TERM_BYTE, ROW_TERM_BYTE
    ];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
        row_count += 1;
    }
    assert!(row_count == 5);

    // 3 rows with multiple strings
    let buffer: Vec<u8> = vec![
        b'a', VALUE_TERM_BYTE, b'a', b'b', b'c', VALUE_TERM_BYTE, b'x', VALUE_TERM_BYTE, ROW_TERM_BYTE,
        ROW_TERM_BYTE,
        b'y', VALUE_TERM_BYTE, b'x', b'z', VALUE_TERM_BYTE, ROW_TERM_BYTE
    ];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<String>>() {
        assert!(result.is_ok());
        let value = result.unwrap();
        if row_count == 1 {
            assert!(value.len() == 0);
        } else {
            assert!(value.len() > 1);
        }
        row_count += 1;
    }
    assert!(row_count == 3);

    // 2 rows with null values
    /* let buffer: Vec<u8> = vec![NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE, NULL_BYTE, VALUE_TERM_BYTE, ROW_TERM_BYTE];
    let mut row_count = 0;
    for result in reader::Reader::from_reader(&*buffer).deserialize::<Vec<NoValue>>() {
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 1);
        row_count += 1;
    }
    assert!(row_count == 2); */
}