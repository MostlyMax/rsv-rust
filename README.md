# rsv
an rsv reader and writer crate for Rust.

### what is rsv?
rows of string values (rsv) is a modification on the csv format that replaces
delimiter characters with unused unicode bytes. This makes encoding and decoding
incredibly simple and consistent.

find the specification created by Stenway [here](https://github.com/Stenway/RSV-Specification)
