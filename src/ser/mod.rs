use std::io::Write;

use serde::ser::{Impossible, Serialize};

use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

use super::error::{self, Result, Error};

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
}

impl<W: Write> Serializer<W> {
    fn new_from_writer(writer: EventWriter<W>) -> Self {
        Self { writer, root: true, current_tag: "".to_string() }
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
    
    fn start_tag(&mut self, tag_name: &str) -> Result<()> {
        self.next(XmlEvent::start_element(tag_name).into())
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
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'ser, W>;
    type SerializeStructVariant = StructVariantSerializer<'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok>
    {
        self.characters(&v.to_string())
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
		self.characters(&v.to_string())
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
		self.characters(&v.to_string())
	}
    
    fn serialize_f32(self, v: f32) -> Result<Self::Ok>
	{
		self.serialize_f64(f64::from(v))
	}
    
    fn serialize_f64(self, v: f64) -> Result<Self::Ok>
	{
		self.characters(&v.to_string())
	}
    
    fn serialize_char(self, v: char) -> Result<Self::Ok>
	{
		self.characters(&v.to_string())
	}
    
    fn serialize_str(self, v: &str) -> Result<Self::Ok>
	{
        self.characters(v)
	}
    
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    
    fn serialize_none(self) -> Result<Self::Ok>
	{
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
        debug!("Newtype variant {}", variant);
        self.start_tag(variant)?;
        value.serialize(&mut *self)?;
        self.end_tag()?;
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
		Ok(TupleSerializer::new(self))
	}
    
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleStruct>
	{
		Ok(TupleSerializer::new(self))
	}
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleVariant>
	{
		self.start_tag(variant)?;
        Ok(TupleSerializer::new(self))
	}
    
    fn serialize_map(
        self,
        len: Option<usize>
    ) -> Result<Self::SerializeMap>
	{
		Ok(self)
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
            self.start_tag(name)?;
            Ok(StructSerializer::new_root(self))
        } else {
            Ok(StructSerializer::new(self))
        }
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
        }
        self.start_tag(variant)?;
        Ok(StructVariantSerializer::new(self))
    }
}

impl<'ser, W: Write> serde::ser::SerializeMap for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

pub struct StructSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    root: bool,
}

impl<'ser, W: 'ser + Write> StructSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        StructSerializer { ser, root: false }
    }
    
    fn new_root(ser: &'ser mut Serializer<W>) -> Self {
        StructSerializer { ser, root: true }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStruct for StructSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_tag = key.to_string();
        debug!("field {}", key);
        self.ser.start_tag(key)?;
        value.serialize(&mut *self.ser)?;
        debug!("end field");
        self.ser.end_tag()?;
        Ok(())
    }
    
    fn end(self) -> Result<()> {
        if self.root {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

pub struct StructVariantSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
}

impl<'ser, W: 'ser + Write> StructVariantSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        Self { ser }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStructVariant for StructVariantSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.current_tag = key.to_string();
        debug!("field {}", key);
        self.ser.start_tag(key)?;
        value.serialize(&mut *self.ser)?;
        debug!("end field");
        self.ser.end_tag()?;
        Ok(())
    }
    
    fn end(self) -> Result<()> {
        self.ser.end_tag()?;
        Ok(())
    }
}

pub struct TupleSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    first: bool,
}

impl<'ser, W: 'ser + Write> TupleSerializer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        Self { ser, first: true }
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
        Ok(())
    }
}

pub struct SeqSeralizer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    tag_name: String,
    first: bool,
}

impl<'ser, W: 'ser + Write> SeqSeralizer<'ser, W> {
    fn new(ser: &'ser mut Serializer<W>) -> Self {
        let tag_name = ser.current_tag();
        SeqSeralizer { ser, tag_name, first: true }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeSeq for SeqSeralizer<'ser, W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.first {
            self.first = false;
        } else {
            self.ser.end_tag()?;
            self.ser.start_tag(&self.tag_name)?;
        }
        value.serialize(&mut *self.ser)?;
        Ok(())
    }
    
    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;