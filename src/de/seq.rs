use std::io::Read;

use xml::reader::XmlEvent;

use super::Deserializer;
use super::super::error::{Error, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    tag_name: Option<String>,
    first: bool,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let tag_name = de.current_tag();
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
            if let Some(ref t) = self.tag_name {
                self.de.end_tag(&t)?;
            }
            
            match (self.de.peek()?.clone(), self.tag_name.as_ref()) {
                (XmlEvent::StartElement { ref name, .. }, Some(t)) if &name.local_name == t => {
                    self.de.start_tag()?;
                    let v = seed.deserialize(&mut *self.de)?;
                    Ok(Some(v))
                },
                _ => Ok(None),
            }
        }
    }
}
