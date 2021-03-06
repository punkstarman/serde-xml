use std::io::Read;

use serde::de::IntoDeserializer;

use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use super::{Deserializer, qualified_name_from};
use super::plain::PlainStringDeserializer;
use super::super::error::{self, Error, Result};

pub struct MapAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    attributes: std::vec::IntoIter<OwnedAttribute>,
    value: Option<String>,
    end_tag: Option<OwnedName>,
}

impl<'a, R: 'a + Read> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attributes: Vec<OwnedAttribute>) -> Self {
        MapAccess {
            de,
            attributes: attributes.into_iter(),
            value: None,
            end_tag: None
        }
    }
}

impl<'a, 'de, R: 'a + Read> serde::de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match self.attributes.next() {
            Some(OwnedAttribute { name, value }) => {
                trace!("found attribute {} {}", name, value);
                self.value = Some(value);
                let attribute_name = format!("@{}", qualified_name_from(&name));
                seed.deserialize(attribute_name.into_deserializer()).map(Some)
            },
            None => match self.de.peek()? {
                XmlEvent::EndElement { .. } | XmlEvent::EndDocument => Ok(None),
                XmlEvent::Characters { .. } => {
                    self.value = Some(self.de.characters()?);
                    seed.deserialize(".".into_deserializer()).map(Some)
                },
                XmlEvent::StartElement { .. } => {
                    let (tag_name, attributes) = self.de.start_tag()?;
                    self.de.tag_name = Some(tag_name.clone());
                    self.de.put_attributes(attributes);
                    self.end_tag = Some(tag_name.clone());

                    let qualified_tag = qualified_name_from(&tag_name);
                    trace!("found subtag {}", qualified_tag);
                    seed.deserialize(qualified_tag.into_deserializer()).map(Some)
                },
                _ => Err(error::with_message(format!("expected map key, found {:?}", self.de.next()?))),
            },
        }
    }

    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        match self.value.take() {
            Some(v) => seed.deserialize(PlainStringDeserializer(v)),
            None => {
                let v = seed.deserialize(&mut *self.de)?;
                let _ = self.de.end_tag(&self.end_tag.take().unwrap());
                Ok(v)
            }
        }
    }
}
