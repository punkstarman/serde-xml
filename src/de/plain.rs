use serde::de::Visitor;

use super::super::error::{self, Error, Result};

pub struct PlainStringDeserializer(pub String);

impl PlainStringDeserializer {
    fn characters(self) -> Result<String> {
        Ok(self.0)
    }
}

macro_rules! deserialize_attr_type {
    ($deserialize:ident, $error:expr => $visit:ident) => {
        fn $deserialize<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            let value = self.characters()?.parse().map_err($error)?;
            visitor.$visit(value)
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for PlainStringDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.characters()?)
    }

    deserialize_attr_type!(deserialize_bool, error::parse_bool => visit_bool);
    deserialize_attr_type!(deserialize_i8, error::parse_int => visit_i8);
    deserialize_attr_type!(deserialize_i16, error::parse_int => visit_i16);
    deserialize_attr_type!(deserialize_i32, error::parse_int => visit_i32);
    deserialize_attr_type!(deserialize_i64, error::parse_int => visit_i64);

    serde_if_integer128! {
        deserialize_attr_type!(deserialize_i128, error::parse_int => visit_i128);
    }

    deserialize_attr_type!(deserialize_u8, error::parse_int => visit_u8);
    deserialize_attr_type!(deserialize_u16, error::parse_int => visit_u16);
    deserialize_attr_type!(deserialize_u32, error::parse_int => visit_u32);
    deserialize_attr_type!(deserialize_u64, error::parse_int => visit_u64);

    serde_if_integer128! {
        deserialize_attr_type!(deserialize_u128, error::parse_int => visit_u128);
    }

    deserialize_attr_type!(deserialize_f32, error::parse_float => visit_f32);
    deserialize_attr_type!(deserialize_f64, error::parse_float => visit_f64);

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    forward_to_deserialize_any! {
        char str string unit seq bytes map unit_struct newtype_struct tuple_struct
        struct identifier tuple ignored_any byte_buf
    }
}
