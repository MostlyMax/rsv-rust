use std::{fs::File, io::{BufRead, BufReader, Read}, marker::PhantomData, path::Path};

use serde::de::DeserializeOwned;
use crate::deserializer::DeRecord;

use crate::error::{ErrorKind, Error};
use crate::utils::ROW_TERM_BYTE;

pub struct Reader<R: Read> {
    rdr: BufReader<R>,
}

impl<R: Read> Reader<R> {
    /// Creates a new buffered reader from any struct that implements the Read trait.
    pub fn from_reader(rdr: R) -> Reader<R> {
        let rdr = BufReader::new(rdr);
        Reader {
            rdr,
        }
    }
}

impl Reader<File> {
    /// Creates a new buffered reader from a file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<File>, Error> {
        let f = File::open(path)?;

        Ok(Reader::from_reader(f))
    }
}

impl<'a, R: Read> Reader<R> {
    /// Returns an iterator that will continuously decode records from the underlying reader.
    pub fn deserialize<'de, D: DeserializeOwned>(&mut self) -> DesRecordIter<'_, D, R> {

        DesRecordIter::<D, R> {
            _priv: PhantomData,
            rdr: self,
        }
    }

    pub fn read_record(&mut self, mut buf: &mut Vec<u8>) -> Option<Result<(), Error>> {
        match self.rdr.read_until(ROW_TERM_BYTE, &mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(())),
            Err(_) => Some(Err(Error(ErrorKind::Deserialize(
                "Failed to read record".to_owned()
            )))),
        }
    }


}


pub struct DesRecordIter<'a, D: DeserializeOwned, R: Read> {
    rdr: &'a mut Reader<R>,
    _priv: PhantomData<D>
}

impl<D: DeserializeOwned, R: Read> Iterator for DesRecordIter<'_, D, R> {
    type Item = Result<D, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();

        match self.rdr.read_record(&mut buf) {
            None => None,
            Some(Ok(())) => {
                let mut d = DeRecord::from_ref(&buf);
                Some(D::deserialize(&mut d))
            },
            Some(Err(e)) => Some(Err(e))
        }
    }
}
