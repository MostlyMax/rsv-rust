use std::{fs::File, io::{BufWriter, Write}, path::Path};
use crate::utils::{NULL_BYTE, ROW_TERM_BYTE, VALUE_TERM_BYTE};

use serde::Serialize;
use crate::serializer::SerRecord;

use crate::error::Error;

pub struct Writer<W> where W: Write {
    wtr: W,
}

impl Writer<BufWriter<File>> {
    /// Creates a new buffered writer from a file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<BufWriter<File>>, Error> {
        let f = File::create(path)?;

        Ok(Writer::from_writer(f))
    }
}

impl<W: Write> Writer<BufWriter<W>> {
    /// Creates a new buffered writer from a struct that implements the Write trait.
    pub fn from_writer(wtr: W) -> Writer<BufWriter<W>> {
        let wtr = BufWriter::new(wtr);
        Writer {
            wtr,
        }
    }
}

impl<W: Write> Writer<W> {
    /// Creates a new *unbuffered* writer from a struct that implements the Write trait.
    pub fn from_writer_unbuffered(wtr: W) -> Self {
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

    /// Encode an iterable struct of items into a single record.
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
        self.wtr.write_all(&[VALUE_TERM_BYTE])?;

        Ok(())
    }

    pub(crate) fn write_null(&mut self) -> Result<(), Error> {
        self.write_value(&[NULL_BYTE])
    }

    pub(crate) fn write_row_term(&mut self) -> Result<(), Error> {
        self.wtr.write_all(&[ROW_TERM_BYTE])?;
        Ok(())
    }

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
