use std::io::Read;

use super::Deserializer;
use super::super::error::{self, Error, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    tag_name: String,
    first: bool,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let tag_name = de.current_tag()
            .ok_or(error::with_message("expected current tag".to_string()))?;
        Ok(SeqAccess { de, tag_name, first: true })
    }
}

impl<'de, 'a, R: 'a + Read> serde::de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>>
    {
        if self.first {
            debug!("beginning of sequence of {}", self.tag_name);
            self.first = false;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            self.de.end_tag(&self.tag_name)?;

            if self.de.is_peek_start_element(&self.tag_name) {
                debug!("element of sequence of {}", self.tag_name);
                self.de.start_tag()?;
                let v = seed.deserialize(&mut *self.de)?;
                Ok(Some(v))
            } else {
                debug!("end of sequence of {}", self.tag_name);
                Ok(None)
            }
        }
    }
}
