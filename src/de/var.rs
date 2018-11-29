use std::io::Read;

use serde::de::{
    Deserializer as SerdeDeserializer,
    IntoDeserializer,
    Visitor};

use super::Deserializer;
use super::super::error::{self, Error, Result};


pub struct VariantAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a + Read> VariantAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        VariantAccess { de: de }
    }
}

impl<'de, 'a, R: 'a + Read> serde::de::EnumAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let v = seed.deserialize(self.de.current_tag()?.into_deserializer())?;
        Ok((v, self))
    }
}

impl<'de, 'a, R: 'a + Read> serde::de::VariantAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        // Handled by UnitVariantAccess
        unimplemented!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_map(visitor)
    }
}

pub struct UnitVariantAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a + Read> UnitVariantAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        UnitVariantAccess { de: de }
    }
}

impl<'de, 'a, R: 'a + Read> serde::de::EnumAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(&mut *self.de)?;
        Ok((variant, self))
    }
}

impl<'de, 'a, R: 'a + Read> serde::de::VariantAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }
}
