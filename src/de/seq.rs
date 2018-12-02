use std::io::Read;

use xml::reader::XmlEvent;

use super::Deserializer;
use super::super::error::{Error, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    tag_name: String,
    first: bool,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let tag_name = de.current_tag()?;
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
            debug!("found sequence tag {}", self.tag_name);
            let v = seed.deserialize(&mut *self.de)?;
            self.de.end_tag(&self.tag_name)?;
            debug!("end sequence tag {}", self.tag_name);
            Ok(Some(v))
        } else {
            match self.de.peek()?.clone() {
                XmlEvent::StartElement { ref name, .. } if name.local_name == self.tag_name => {
                    self.de.start_tag()?;
                    debug!("found sequence tag {}", self.tag_name);
                    let v = seed.deserialize(&mut *self.de)?;
                    self.de.end_tag(&self.tag_name)?;
                    debug!("end sequence tag {}", self.tag_name);
                    Ok(Some(v))
                },
                _ => Ok(None),
            }
        }
    }
}
