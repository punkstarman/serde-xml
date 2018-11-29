use std::io::Read;

use serde::de::IntoDeserializer;

use super::Deserializer;
use super::super::error::{Error, Result};

pub struct TupleAccess {
    items: ::std::vec::IntoIter<String>,
}

impl TupleAccess {
    pub fn new<'a, R: 'a + Read>(de: &'a mut Deserializer<R>) -> Result<Self> {
        let items: Vec<String> = de.characters()?.split_whitespace()
            .map(String::from)
            .collect();
        Ok(TupleAccess { items: items.into_iter() })
    }
}

impl<'de> serde::de::SeqAccess<'de> for TupleAccess {
    type Error = Error;
    
    fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>>
    {
        match self.items.next() {
            None => Ok(None),
            Some(item) => {
                seed.deserialize(item.into_deserializer()).map(Some)
            },
        }
    }
}
