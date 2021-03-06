use std::io::Read;

use xml::name::OwnedName;
use xml::reader::XmlEvent;

use super::Deserializer;
use super::super::error::{self, Error, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    tag_name: OwnedName,
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
            self.first = false;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            self.de.end_tag(&self.tag_name)?;
            match self.de.peek()?.clone() {
                XmlEvent::StartElement { ref name, .. } if name == &self.tag_name => {
                    self.de.start_tag()?;
                    let v = seed.deserialize(&mut *self.de)?;
                    Ok(Some(v))
                },
                _ => Ok(None),
            }
        }
    }
}
