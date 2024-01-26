use std::{fs::File, io::{BufWriter, Write}, path::Path};

#[cfg(features = "serde")]
use serde::{ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}, Serialize, Serializer};

#[cfg(features = "serde")]
use crate::serializer::SerRecord;

use crate::error::Error;

pub struct Writer<W> where W: Write {
    wtr: W,
}

impl Writer<BufWriter<File>> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<BufWriter<File>>, Error> {
        let f = File::create(path)?;
        let wtr = BufWriter::new(f);

        Ok(Writer::from_writer(wtr))
    }
}

impl<W: Write> Writer<W> {
    const NULL_BYTE: u8 = 0xFE;
    const ROW_TERM_BYTE: u8 = 0xFD;
    const VALUE_TERM_BYTE: u8 = 0xFF;

    pub fn from_writer(wtr: W) -> Self {
        Writer {
            wtr,
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        Ok(self.wtr.flush()?)
    }

    pub fn get_ref(&self) -> &W {
        &self.wtr
    }

    // pub fn into_inner(&self) -> Result<W, IntoInnerError<Writer<W>>> {
    //     match self.flush() {
    //         Ok(_) => Ok(self.wtr),
    //         Err(e) => Err(IntoInnerError::new(self, e))
    //     }
    // }

    pub fn write_record<'r, I, T>(&mut self, rec: I) -> Result<(), Error>
        where I: IntoIterator<Item = &'r Option<T>>, T: AsRef<[u8]> + 'r {

        for v in rec {
            match v {
                Some(v) => self.write_value(v)?,
                None => self.write_null()?
            }
        }

        self.write_row_term()?;

        Ok(())
    }

    pub(crate) fn write_value<T: AsRef<[u8]>>(&mut self, value: T) -> Result<(), Error> {
        self.wtr.write_all(value.as_ref())?;
        self.wtr.write_all(&[Self::VALUE_TERM_BYTE])?;

        Ok(())
    }

    pub(crate) fn write_null(&mut self) -> Result<(), Error> {
        self.write_value(&[Self::NULL_BYTE])
    }

    pub(crate) fn write_row_term(&mut self) -> Result<(), Error> {
        self.wtr.write_all(&[Self::ROW_TERM_BYTE])?;
        Ok(())
    }

    #[cfg(features = "serde")]
    pub fn serialize<S: Serialize>(&mut self, record: S) -> Result<(), Error> {
        record.serialize(&mut SerRecord { wtr: self })?;
        self.write_row_term()?;
        Ok(())
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}