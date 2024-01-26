use std::{iter, str::from_utf8};

use serde::{de::SeqAccess, Deserializer};

use crate::error::{Error, ErrorKind};

pub struct DeRecord<'de> {
    buf: &'de [u8],
}

impl<'de> DeRecord<'de> {
    const NULL_BYTE: u8 = 0xFE;
    const ROW_TERM_BYTE: u8 = 0xFD;
    const VALUE_TERM_BYTE: u8 = 0xFF;

    pub(crate) fn from_ref(buf: &'de [u8]) -> Self {
        DeRecord { buf }
    }

    fn next_is_null(&mut self) -> Result<bool, Error> {
        match self.buf[0] == Self::NULL_BYTE {
            true => {
                if self.buf[1] == Self::VALUE_TERM_BYTE {
                    self.buf = &self.buf[2..];
                    Ok(true)
                } else {
                    Err(Error(ErrorKind::Deserialize(
                        format!("Expected TERM_BYTE after NULL_BYTE, got {:?}", self.buf[1])
                    )))
                }
            },
            false => Ok(false),
        }
    }

    fn next_value(&mut self) -> Result<Option<&str>, Error> {
        // Check if value is null byte
        if self.buf[0] == Self::NULL_BYTE {
            if self.buf[1] == Self::VALUE_TERM_BYTE {
                self.buf = &self.buf[2..];
                return Ok(None);
            } else {
                return Err(Error(ErrorKind::Deserialize(
                    format!("Expected TERM_BYTE after NULL_BYTE, got {:?}", self.buf[1])
                )));
            }
        }

        // Check if value is empty string
        if self.buf[0] == Self::VALUE_TERM_BYTE {
            self.buf = &self.buf[1..];
            return Ok(Some(""));
        }

        // Parse UTF-8 String
        for i in 1..self.buf.len() {
            if self.buf[i] == Self::VALUE_TERM_BYTE {
                let value = &self.buf[0..i];
                self.buf = &self.buf[i + 1..];

                return Ok(Some(from_utf8(value)?));
            }
        }

        Err(Error(ErrorKind::Deserialize(
            "Unable to find VALUE_TERM_BYTE in record".to_owned()
        )))
    }

    fn next_str_value(&mut self) -> Result<&str, Error> {
        let value = self.next_value()?;
        let value = value.ok_or(Error(ErrorKind::Deserialize(
            "Got None but expected string".to_owned()
        )))?;

        Ok(value)
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut DeRecord<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;

        match value {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(Error(ErrorKind::Deserialize("Failed to deserialize bool".to_owned()))),
        }
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_u8(value)}

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_u16(value)}

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        let value = value.parse().unwrap();

        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        if value.len() <= 1 {
            return Err(Error(ErrorKind::Deserialize(
                format!("Expected 1 character, got {}", value.len())
            )));
        }
        let value = value.chars().next().unwrap_or_default();
        visitor.visit_char(value)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        visitor.visit_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_str_value()?;
        visitor.visit_string(value.to_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.next_is_null()?;
        match value {
            true => visitor.visit_none(),
            false => visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_seq(self)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_seq(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("i128 is not supported"))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("u128 is not supported"))
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'a, 'de> SeqAccess<'de> for DeRecord<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {
        if self.buf[0] == DeRecord::ROW_TERM_BYTE {
            return Ok(None);
        }

        seed.deserialize(self).map(Some)
    }
}
