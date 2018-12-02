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
}

impl<W: Write> Serializer<W> {
    fn new_from_writer(writer: EventWriter<W>) -> Self {
        Self { writer, root: true }
    }
    
    pub fn new(writer: W) -> Self {
        Self::new_from_writer(EmitterConfig::new()
            .perform_indent(true)
            .keep_element_names_stack(true)
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
}

#[allow(unused_variables)]
impl<'ser, W: Write> serde::ser::Serializer for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'ser, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok>
    {
        unimplemented!()
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok>
    {
        unimplemented!()
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_i32(self, v: i32) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_i64(self, v: i64) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_u8(self, v: u8) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_u16(self, v: u16) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_u32(self, v: u32) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_u64(self, v: u64) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_f32(self, v: f32) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_f64(self, v: f64) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_char(self, v: char) -> Result<Self::Ok>
	{
		unimplemented!()
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
		unimplemented!()
	}
    fn serialize_some<T: ?Sized>(
        self,
        value: &T
    ) -> Result<Self::Ok>
    where T: Serialize
    {
        unimplemented!()
    }
    fn serialize_unit(self) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_unit_struct(
        self,
        name: &'static str
    ) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str
    ) -> Result<Self::Ok>
	{
		unimplemented!()
	}
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T
    ) -> Result<Self::Ok>
    where
        T: Serialize
	{
		unimplemented!()
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
		unimplemented!()
	}
    fn serialize_seq(
        self,
        len: Option<usize>
    ) -> Result<Self::SerializeSeq>
	{
		unimplemented!()
	}
    fn serialize_tuple(
        self,
        len: usize
    ) -> Result<Self::SerializeTuple>
	{
		unimplemented!()
	}
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleStruct>
	{
		unimplemented!()
	}
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleVariant>
	{
		unimplemented!()
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
            Ok(StructSerializer { ser: self, root: true })
        } else {
            Ok(StructSerializer { ser: self, root: false })
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'ser, W: Write> serde::ser::SerializeMap for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
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

impl<'ser, W: Write> serde::ser::SerializeStruct for StructSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;
    
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
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

#[cfg(test)]
mod tests;