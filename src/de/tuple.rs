use std::io::Read;

use super::Deserializer;
use super::super::error::{self, Error, Result};

pub struct TupleAccess {
    items: ::std::vec::IntoIter<String>,
}

impl TupleAccess {
    pub fn new<'a, R: 'a + Read>(de: &'a mut Deserializer<R>) -> Result<Self> {
        trace!("looking for tuple");
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
                trace!("found tuple item {}", item);
                seed.deserialize(TupleValueDeserializer(item)).map(Some)
            },
        }
    }
}

struct TupleValueDeserializer(String);

macro_rules! deserialize_type_tuple {
    ($deserialize:ident, $error:expr => $visit:ident) => {
        fn $deserialize<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            visitor.$visit(self.0.parse().map_err($error)?)
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for TupleValueDeserializer {
    type Error = Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.0)
    }

    deserialize_type_tuple!(deserialize_i8, error::parse_int => visit_i8);
    deserialize_type_tuple!(deserialize_i16, error::parse_int => visit_i16);
    deserialize_type_tuple!(deserialize_i32, error::parse_int => visit_i32);
    deserialize_type_tuple!(deserialize_i64, error::parse_int => visit_i64);
    deserialize_type_tuple!(deserialize_u8, error::parse_int => visit_u8);
    deserialize_type_tuple!(deserialize_u16, error::parse_int => visit_u16);
    deserialize_type_tuple!(deserialize_u32, error::parse_int => visit_u32);
    deserialize_type_tuple!(deserialize_u64, error::parse_int => visit_u64);
    deserialize_type_tuple!(deserialize_f32, error::parse_float => visit_f32);
    deserialize_type_tuple!(deserialize_f64, error::parse_float => visit_f64);

    forward_to_deserialize_any! {
        char str string unit seq bytes map unit_struct newtype_struct tuple_struct
        struct identifier tuple ignored_any byte_buf enum option bool
    }
}
