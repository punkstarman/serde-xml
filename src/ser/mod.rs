mod map;
mod plain;
mod seq;
mod tuple;

use std::collections::HashMap;
use std::io::Write;

use regex::Regex;

use serde::ser::Serialize;

use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

use super::error::{self, Result, Error};

use self::map::{MapSerializer, StructSerializer};
use self::seq::SeqSeralizer;
use self::tuple::TupleSerializer;

pub fn to_string<S: Serialize>(value: &S) -> Result<String> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;

    let string = String::from_utf8(writer).map_err(error::from_utf8)?;
    Ok(string)
}

pub fn to_writer<W: Write, S: Serialize>(writer: W, value: &S) -> Result<()> {
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

pub struct Serializer<W>
where W: Write {
    writer: EventWriter<W>,
    root: bool,
    current_tag: String,
    current_tag_attrs: Option<HashMap<&'static str, String>>,
}

impl<W: Write> Serializer<W> {
    fn new_from_writer(writer: EventWriter<W>) -> Self {
        Self { writer, root: true, current_tag: "".into(), current_tag_attrs: None }
    }

    pub fn new(writer: W) -> Self {
        Self::new_from_writer(
            EmitterConfig::new()
            .perform_indent(true)
            .create_writer(writer))
    }

    fn next(&mut self, event: XmlEvent) -> Result<()> {
        self.writer.write(event).map_err(error::writer)
    }

    fn characters(&mut self, s: &str) -> Result<()> {
    	self.next(XmlEvent::characters(s))
    }

    fn start_document(&mut self) -> Result<()> {
        self.next(XmlEvent::StartDocument {
            encoding: Default::default(),
            standalone: Default::default(),
            version: xml::common::XmlVersion::Version10
        })
    }

    fn open_root_tag(&mut self, name: &'static str) -> Result<()> {
        if self.root {
            self.root = false;
            self.start_document()?;
            self.open_tag(name)?;
        }
        Ok(())
    }

    fn open_tag(&mut self, tag_name: &str) -> Result<()> {
        self.current_tag = tag_name.into();
        self.current_tag_attrs = Some(HashMap::new());
        Ok(())
    }

    fn reopen_tag(&mut self) -> Result<()> {
        self.current_tag_attrs = Some(HashMap::new());
        Ok(())
    }

    fn abandon_tag(&mut self) -> Result<()> {
        self.current_tag = "".into();
        self.current_tag_attrs = None;
        Ok(())
    }

    fn add_attr(&mut self, name: &'static str, value: String) -> Result<()> {
        self.current_tag_attrs.as_mut()
            .ok_or(error::with_message("Cannot add attribute".into()))
            .map(|attrs| {
                attrs.insert(name, value);
            })
    }

    fn build_start_tag(&mut self) -> Result<bool> {
        lazy_static! {
            static ref TAG_RE: Regex = Regex::new(r#"((?P<ns>.*):)?(?P<local>[^:]*)"#).unwrap();
        }

        if let Some(attrs) = self.current_tag_attrs.take() {
            let current_tag = self.current_tag();
            let captures = TAG_RE.captures(&current_tag).unwrap();

            let default_ns = captures.name("ns").map(|m| m.as_str());
            let tag = captures.name("local").unwrap().as_str();
            self.start_tag(tag, attrs, default_ns)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn start_tag(
        &mut self,
        tag_name: &str,
        attrs: HashMap<&str, String>,
        default_ns: Option<&str>) -> Result<()>
    {
        let mut element = attrs.iter().fold(
            XmlEvent::start_element(tag_name),
            |b, (&name, value)| b.attr(name, value));

        if default_ns.is_some() {
            element = element.default_ns(default_ns.unwrap());
        }

        self.next(element.into())
    }

    fn end_tag(&mut self) -> Result<()> {
        self.next(XmlEvent::end_element().into())
    }

    fn current_tag(&self) -> String {
        self.current_tag.clone()
    }
}

impl<'ser, W: Write> serde::ser::Serializer for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSeralizer<'ser, W>;
    type SerializeTuple = TupleSerializer<'ser, W>;
    type SerializeTupleStruct = TupleSerializer<'ser, W>;
    type SerializeTupleVariant = TupleSerializer<'ser, W>;
    type SerializeMap = MapSerializer<'ser, W>;
    type SerializeStruct = StructSerializer<'ser, W>;
    type SerializeStructVariant = StructSerializer<'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok>
    {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok>
    {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok>
	{
		self.serialize_i64(i64::from(v))
	}

    fn serialize_i32(self, v: i32) -> Result<Self::Ok>
	{
		self.serialize_i64(i64::from(v))
	}

    fn serialize_i64(self, v: i64) -> Result<Self::Ok>
	{
        self.serialize_str(&v.to_string())
	}

    fn serialize_u8(self, v: u8) -> Result<Self::Ok>
	{
		self.serialize_u64(u64::from(v))
	}

    fn serialize_u16(self, v: u16) -> Result<Self::Ok>
	{
		self.serialize_u64(u64::from(v))
	}

    fn serialize_u32(self, v: u32) -> Result<Self::Ok>
	{
		self.serialize_u64(u64::from(v))
	}

    fn serialize_u64(self, v: u64) -> Result<Self::Ok>
	{
        let must_close_tag = self.build_start_tag()?;
        self.characters(&v.to_string())?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
	}

    fn serialize_f32(self, v: f32) -> Result<Self::Ok>
	{
		self.serialize_f64(f64::from(v))
	}

    fn serialize_f64(self, v: f64) -> Result<Self::Ok>
	{
        self.serialize_str(&v.to_string())
	}

    fn serialize_char(self, v: char) -> Result<Self::Ok>
	{
        self.serialize_str(&v.to_string())
	}

    fn serialize_str(self, v: &str) -> Result<Self::Ok>
	{
        let must_close_tag = self.build_start_tag()?;
        self.characters(v)?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
	}

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok>
	{
		unimplemented!()
	}

    fn serialize_none(self) -> Result<Self::Ok>
	{
        trace!("None");
        let must_close_tag = self.build_start_tag()?;
        if must_close_tag {
            self.end_tag()?;
        }
		Ok(())
	}

    fn serialize_some<T: ?Sized>(
        self,
        value: &T
    ) -> Result<Self::Ok>
    where T: Serialize
    {
        trace!("Some");
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok>
	{
        trace!("Unit");
        let must_close_tag = self.build_start_tag()?;
        if must_close_tag {
            self.end_tag()?;
        }
		Ok(())
	}

    fn serialize_unit_struct(
        self,
        name: &'static str
    ) -> Result<Self::Ok>
	{
        trace!("Unit struct {}", name);
		self.serialize_unit()
	}

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str
    ) -> Result<Self::Ok>
	{
        trace!("Unit variant {}::{}", name, variant);
        self.serialize_str(variant)
	}

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T
    ) -> Result<Self::Ok>
    where
        T: Serialize
	{
        trace!("Newtype struct {}", name);
        value.serialize(self)
	}

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> Result<Self::Ok>
    where
        T: Serialize
	{
        let must_close_tag = self.build_start_tag()?;

        trace!("Newtype variant {}::{}", name, variant);
        self.open_tag(variant)?;
        value.serialize(&mut *self)?;

        if must_close_tag {
            self.end_tag()?;
        }
		Ok(())
	}

    fn serialize_seq(
        self,
        _len: Option<usize>
    ) -> Result<Self::SerializeSeq>
	{
        trace!("Sequence");
		Ok(SeqSeralizer::new(self))
	}

    fn serialize_tuple(
        self,
        _len: usize
    ) -> Result<Self::SerializeTuple>
	{
        trace!("Tuple");
        let must_close_tag = self.build_start_tag()?;
		Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleStruct>
	{
        trace!("Tuple struct {}", name);
        let must_close_tag = self.build_start_tag()?;
		Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleVariant>
	{
        trace!("Tuple variant {}::{}", name, variant);
        let must_close_tag = self.build_start_tag()?;
        self.start_tag(variant, HashMap::new(), None)?;
        Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_map(
        self,
        _len: Option<usize>
    ) -> Result<Self::SerializeMap>
	{
        let must_close_tag = self.build_start_tag()?;
		Ok(MapSerializer::new(self, must_close_tag))
	}

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStruct> {
        self.open_root_tag(name)?;

        trace!("Struct {}", name);
        Ok(StructSerializer::new(self, false))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStructVariant> {
        self.open_root_tag(name)?;

        trace!("Struct variant {}", variant);
        let must_close_tag = self.build_start_tag()?;
        self.open_tag(variant)?;
        Ok(StructSerializer::new(self, must_close_tag))
    }
}

#[cfg(test)]
mod tests;
