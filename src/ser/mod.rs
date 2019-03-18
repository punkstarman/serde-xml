mod plain;

use std::collections::HashMap;
use std::io::Write;

use serde::ser::Serialize;

use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

use super::error::{self, Result, Error};

use self::plain::to_plain_string;

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
        Self { writer, root: true, current_tag: "".to_string(), current_tag_attrs: None }
    }

    pub fn new(writer: W) -> Self {
        Self::new_from_writer(EmitterConfig::new()
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
        if let Some(attrs) = self.current_tag_attrs.take() {
            let current_tag = self.current_tag();
            self.start_tag(&current_tag, attrs)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn start_tag(&mut self, tag_name: &str, attrs: HashMap<&str, String>) -> Result<()> {
        let element = attrs.iter().fold(
            XmlEvent::start_element(tag_name),
            |b, (&name, value)| b.attr(name, value));
        self.next(element.into())
    }

    fn end_tag(&mut self) -> Result<()> {
        self.next(XmlEvent::end_element().into())
    }

    fn current_tag(&self) -> String {
        self.current_tag.clone()
    }
}

#[allow(unused_variables)]
impl<'ser, W: Write> serde::ser::Serializer for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSeralizer<'ser, W>;
    type SerializeTuple = TupleSerializer<'ser, W>;
    type SerializeTupleStruct = TupleSerializer<'ser, W>;
    type SerializeTupleVariant = TupleSerializer<'ser, W>;
    type SerializeMap = MapSerializer<'ser, W>;
    type SerializeStruct = StructSerializer<'ser, W>;
    type SerializeStructVariant = StructVariantSerializer<'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok>
    {
        let must_close_tag = self.build_start_tag()?;
        self.characters(&v.to_string())?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
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
        let must_close_tag = self.build_start_tag()?;
        self.characters(&v.to_string())?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
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
        let must_close_tag = self.build_start_tag()?;
        self.characters(&v.to_string())?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
	}

    fn serialize_char(self, v: char) -> Result<Self::Ok>
	{
        let must_close_tag = self.build_start_tag()?;
        self.characters(&v.to_string())?;
        if must_close_tag {
            self.end_tag()?;
        }
        Ok(())
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

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok>
	{
		unimplemented!()
	}

    fn serialize_none(self) -> Result<Self::Ok>
	{
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
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok>
	{
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
		self.serialize_unit()
	}

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str
    ) -> Result<Self::Ok>
	{
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
        value.serialize(self)
	}

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> Result<Self::Ok>
    where
        T: Serialize
	{
        let must_close_tag = self.build_start_tag()?;
        debug!("Newtype variant {}", variant);
        self.open_tag(variant)?;
        //self.start_tag(variant)?;
        value.serialize(&mut *self)?;
        //self.end_tag()?;

        if must_close_tag {
            self.end_tag()?;
        }
		Ok(())
	}

    fn serialize_seq(
        self,
        len: Option<usize>
    ) -> Result<Self::SerializeSeq>
	{
		Ok(SeqSeralizer::new(self))
	}

    fn serialize_tuple(
        self,
        len: usize
    ) -> Result<Self::SerializeTuple>
	{
        let must_close_tag = self.build_start_tag()?;
		Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleStruct>
	{
        let must_close_tag = self.build_start_tag()?;
		Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleVariant>
	{
        let must_close_tag = self.build_start_tag()?;
		//self.start_tag(variant)?;
        self.open_tag(variant)?;
        self.build_start_tag()?;
        Ok(TupleSerializer::new(self, must_close_tag))
	}

    fn serialize_map(
        self,
        len: Option<usize>
    ) -> Result<Self::SerializeMap>
	{
        let must_close_tag = self.build_start_tag()?;
		Ok(MapSerializer::new(self, must_close_tag))
	}

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize
    ) -> Result<Self::SerializeStruct> {
        debug!("Struct {}", name);
        if self.root {
            self.root = false;
            self.start_document()?;
            self.open_tag(name)?;
        }
        Ok(StructSerializer::new(self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeStructVariant> {
        debug!("Struct variant {}", variant);
        if self.root {
            self.root = false;
            self.start_document()?;
            self.open_tag(name)?;
        }
        //self.start_tag(variant)?;
        let must_close_tag = self.build_start_tag()?;
        self.open_tag(variant)?;
        Ok(StructVariantSerializer::new(self, must_close_tag))
    }
}

pub struct MapSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
}

impl<'ser, W: 'ser + Write> MapSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        MapSerializer { ser, must_close_tag }
    }
}

impl<'ser, W: Write> serde::ser::SerializeMap for MapSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        //self.ser.start_tag(&to_plain_string(key)?)?;
        self.ser.open_tag(&to_plain_string(key)?)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)?;
        //self.ser.end_tag()?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

pub struct StructSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>
}

impl<'ser, W: 'ser + Write> StructSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        StructSerializer { ser }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStruct for StructSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if key.starts_with("@") {
            debug!("attribute {}", key);
            self.ser.add_attr(&key[1..], plain::to_plain_string(value)?)
        } else {
            self.ser.build_start_tag()?;
            self.ser.open_tag(key)?;
            debug!("field {}", key);
            value.serialize(&mut *self.ser)?;
            debug!("end field");
            Ok(())
        }
    }

    fn end(self) -> Result<()> {
        self.ser.build_start_tag()?;
        self.ser.end_tag()?;
        Ok(())
    }
}

pub struct StructVariantSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
}

impl<'ser, W: 'ser + Write> StructVariantSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        Self { ser, must_close_tag }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStructVariant for StructVariantSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if key.starts_with("@") {
            debug!("attribute {}", key);
            self.ser.add_attr(&key[1..], plain::to_plain_string(value)?)
        } else {
            self.ser.build_start_tag()?;
            self.ser.open_tag(key)?;
            debug!("field {}", key);
            value.serialize(&mut *self.ser)?;
            debug!("end field");
            Ok(())
        }
    }

    fn end(self) -> Result<()> {
        self.ser.build_start_tag()?;
        self.ser.end_tag()?;
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

pub struct TupleSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
    first: bool,
}

impl<'ser, W: 'ser + Write> TupleSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        Self { ser, must_close_tag, first: true }
    }

    fn serialize_item<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.first {
            self.first = false;
        } else {
            self.ser.characters(" ")?;
        }
        value.serialize(&mut *self.ser)?;
        Ok(())
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTupleVariant for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        self.ser.end_tag()?;
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTupleStruct for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTuple for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

pub struct SeqSeralizer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>
}

impl<'ser, W: 'ser + Write> SeqSeralizer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        SeqSeralizer { ser }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeSeq for SeqSeralizer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let must_close_tag = self.ser.build_start_tag()?;
        value.serialize(&mut *self.ser)?;
        if must_close_tag {
            self.ser.end_tag()?;
            self.ser.reopen_tag()?;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.abandon_tag()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
