use std::io::Read;

use serde::de::{DeserializeOwned, Visitor};

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, ParserConfig, XmlEvent};

use super::error::{self, Error, Result};

mod map;
mod seq;
mod tuple;
mod var;
mod plain;

use self::map::MapAccess;
use self::seq::SeqAccess;
use self::tuple::TupleAccess;
use self::var::{VariantAccess, UnitVariantAccess};

pub fn from_reader<'de, R: Read, T: serde::de::Deserialize<'de>>(reader: R) -> Result<T> {
    T::deserialize(&mut Deserializer::new_from_reader(reader)?)
}

pub fn from_str<T>(s: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    from_reader(s.as_bytes())
}

pub struct Deserializer<R: Read> {
    reader: EventReader<R>,
    root: bool,
    lookahead: Option<XmlEvent>,
    tag_name: Option<String>,
    attributes: Option<Vec<OwnedAttribute>>,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: EventReader<R>) -> Result<Self> {
        let d = Deserializer {
            reader,
            root: true,
            lookahead: None,
            tag_name: None,
            attributes: None,
        };
        Ok(d)
    }

    pub fn new_from_reader(reader: R) -> Result<Self> {
        let config = ParserConfig::new()
            .trim_whitespace(true)
            .whitespace_to_characters(true)
            .cdata_to_characters(true)
            .ignore_comments(true)
            .coalesce_characters(true);

        Self::new(EventReader::new_with_config(reader, config))
    }

    fn peek(&mut self) -> Result<&XmlEvent> {
        trace!("Peeking ...");
        match self.lookahead {
            None => { self.lookahead = Some(self.do_next()?); Ok(&self.lookahead.as_ref().unwrap()) },
            Some(ref e) => Ok(&e)
        }
    }

    fn do_next(&mut self) -> Result<XmlEvent> {
        trace!("Reading from {:p}", &self.reader);
        match self.reader.next().map_err(error::reader)? {
            XmlEvent::ProcessingInstruction { .. } => self.do_next(),
            e => {
                trace!("event {:?}", e);
                Ok(e)
            }
        }
    }

    fn next(&mut self) -> Result<XmlEvent> {
        trace!("Popping!");
        match self.lookahead.take() {
            Some(e) => Ok(e),
            None => self.do_next(),
        }
    }

    fn current_tag(&self) -> Option<String> {
        self.tag_name.as_ref().cloned()
    }

    fn clear_tag(&mut self) -> Result<()> {
        self.tag_name = None;
        Ok(())
    }

    fn take_attributes(&mut self) -> Vec<OwnedAttribute> {
        self.attributes.take().unwrap_or(vec![])
    }

    fn start_document(&mut self) -> Result<()> {
        match self.next()? {
            XmlEvent::StartDocument { .. } => Ok(()),
            e => Err(error::with_message(format!("expected start document, but got {:?}", e))),
        }
    }

    fn end_document(&mut self) -> Result<()> {
        match self.next()? {
            XmlEvent::EndDocument { .. } => Ok(()),
            e => Err(error::with_message(format!("expected end of document, but got {:?}", e))),
        }
    }

    fn start_tag(&mut self) -> Result<(String, Vec<OwnedAttribute>)> {
        match self.next()? {
            XmlEvent::StartElement { name, attributes, .. } => Ok((name.local_name, attributes)),
            _ => Err(error::with_message("expecting start tag".to_string())),
        }
    }

    fn end_tag(&mut self, tag_name: &str) -> Result<()> {
        match self.next()? {
            XmlEvent::EndElement { ref name } if name.to_string() == tag_name => Ok(()),
            _ => Err(error::with_message("expecting end tag".to_string())),
        }
    }

    fn characters(&mut self) -> Result<String> {
        trace!("looking for characters");
        match self.next()? {
            XmlEvent::Characters(s) => { trace!("got characters {}", s); Ok(s)},
            e => Err(error::with_message(format!("expecting characters but got {:?}", e))),
        }
    }
}

macro_rules! deserialize_type {
    ($deserialize:ident, $error:expr => $visit:ident) => {
        fn $deserialize<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            let value = self.characters()?.parse().map_err($error)?;
            visitor.$visit(value)
        }
    }
}

impl<'de, 'r, R: Read> serde::de::Deserializer<'de> for &'r mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.peek()? {
            XmlEvent::StartElement { .. } => self.deserialize_map(visitor),
            XmlEvent::EndElement { .. } => self.deserialize_unit(visitor),
            _ => self.deserialize_string(visitor),
        }
    }

    deserialize_type!(deserialize_bool, error::parse_bool => visit_bool);
    deserialize_type!(deserialize_i8, error::parse_int => visit_i8);
    deserialize_type!(deserialize_i16, error::parse_int => visit_i16);
    deserialize_type!(deserialize_i32, error::parse_int => visit_i32);
    deserialize_type!(deserialize_i64, error::parse_int => visit_i64);

    serde_if_integer128! {
        deserialize_type!(deserialize_i128, error::parse_int => visit_i128);
    }

    deserialize_type!(deserialize_u8, error::parse_int => visit_u8);
    deserialize_type!(deserialize_u16, error::parse_int => visit_u16);
    deserialize_type!(deserialize_u32, error::parse_int => visit_u32);
    deserialize_type!(deserialize_u64, error::parse_int => visit_u64);

    serde_if_integer128! {
        deserialize_type!(deserialize_u128, error::parse_int => visit_u128);
    }

    deserialize_type!(deserialize_f32, error::parse_float => visit_f32);
    deserialize_type!(deserialize_f64, error::parse_float => visit_f64);

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.characters()?;
        visitor.visit_string(s)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek()? {
            XmlEvent::EndElement { .. } => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self)?)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(TupleAccess::new(self)?)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.clear_tag()?;
        visitor.visit_seq(TupleAccess::new(self)?)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        trace!("Map");
        if self.root {
            self.root = false;
            self.start_document()?;
            let (tag_name, attributes) = self.start_tag()?;

            let v = visitor.visit_map(MapAccess::new(&mut self, attributes))?;

            let _ = self.end_tag(&tag_name);
            self.end_document()?;
            Ok(v)
        } else {
            let attributes = self.take_attributes();
            visitor.visit_map(MapAccess::new(&mut self, attributes))
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        trace!("Struct {}", name);
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek()? {
            XmlEvent::StartElement { .. } => {
                let (tag_name, _) = self.start_tag()?;
                self.tag_name = Some(tag_name.clone());
                trace!("Variant {}", tag_name);
                let v = visitor.visit_enum(VariantAccess::new(self)?)?;
                self.end_tag(&tag_name)?;
                Ok(v)
            },
            XmlEvent::Characters { .. } => visitor.visit_enum(UnitVariantAccess::new(self)),
            _ => Err(error::with_message("expected enum value".to_string())),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[cfg(test)]
mod tests;
