use std::io::Read;

use serde::de::IntoDeserializer;

use xml::reader::XmlEvent;

use super::Deserializer;
use super::super::error::{self, Error, Result};

pub struct MapAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    end_tag: Option<String>,
}

impl<'a, R: 'a + Read> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        MapAccess { de, end_tag: None }
    }
}

impl<'a, 'de, R: 'a + Read> serde::de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;
    
    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match self.de.peek()? {
            XmlEvent::EndElement { .. } | XmlEvent::EndDocument => Ok(None),
            XmlEvent::StartElement { .. } => {
                let tag_name = self.de.start_tag()?;
                self.de.tag_name = tag_name.clone();
                println!("  found subtag {}", tag_name);
                self.end_tag = Some(tag_name.clone());
                seed.deserialize(tag_name.into_deserializer()).map(Some)
            },
            _ => Err(error::with_message(format!("expected map key, found {:?}", self.de.next()?))),
        }
    }
    
    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        let v = seed.deserialize(&mut *self.de)?;
        self.de.end_tag(self.end_tag.take().unwrap()).ok();
        Ok(v)
    }
}
