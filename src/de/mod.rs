use std::io::Read;

use serde::de::{
    self, DeserializeOwned, Visitor, IntoDeserializer,
    Deserializer as SerdeDeserializer,
};

use xml::reader::{EventReader, ParserConfig, XmlEvent};

use super::error::{self, Error, Result};

mod map;
mod seq;

use self::map::MapAccess;
use self::seq::SeqAccess;

pub fn from_reader<'de, R: Read, T: de::Deserialize<'de>>(reader: R) -> Result<T> {
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
    lookahead: Option<XmlEvent>,
    tag_name: String,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: EventReader<R>) -> Result<Self> {
        let mut d = Deserializer {
            reader,
            lookahead: None,
            tag_name: "".to_string(),
        };
        d.start_document()?;
        d.start_tag()?;
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
        println!("Peeking ...");
        match self.lookahead {
            None => { self.lookahead = Some(self.do_next()?); Ok(&self.lookahead.as_ref().unwrap()) },
            Some(ref e) => Ok(&e)
        }
    }
    
    fn do_next(&mut self) -> Result<XmlEvent> {
        println!("Reading from {:p}", &self.reader);
        match self.reader.next().map_err(error::reader)? {
            XmlEvent::ProcessingInstruction { .. }
            | XmlEvent::Comment(_) => self.do_next(),
            e => {
                println!("  event {:?}", e);
                Ok(e)
            }
        }
    }
    
    fn next(&mut self) -> Result<XmlEvent> {
        println!("Popping!");
        match self.lookahead.take() {
            Some(e) => Ok(e),
            None => self.do_next(),
        }
    }
    
    fn current_tag(&self) -> Result<String> {
        Ok(self.tag_name.clone())
    }
    
    fn start_document(&mut self) -> Result<()> {
        match self.next()? {
            XmlEvent::StartDocument { .. } => Ok(()),
            e => Err(error::with_message(format!("expected start document, but got {:?}", e)))
        }
    }
    
    fn start_tag(&mut self) -> Result<String> {
        match self.next()? {
            XmlEvent::StartElement { name, .. } => Ok(name.local_name),
            _ => Err(error::with_message("expecting start tag".to_string())),
        }
    }
    
    fn end_tag(&mut self, tag_name: String) -> Result<()> {
        match self.next()? {
            XmlEvent::EndElement { ref name } if name.to_string() == tag_name => Ok(()),
            _ => Err(error::with_message("expecting end tag".to_string())),
        }
    }
    
    fn characters(&mut self) -> Result<String> {
        println!("looking for characters");
        match self.next()? {
            XmlEvent::Characters(s) => { println!("got {}", s); Ok(s)},
            e => Err(error::with_message(format!("expecting characters but got {:?}", e))),
        }
    }
}

impl<'de, 'r, R: Read> de::Deserializer<'de> for &'r mut Deserializer<R> {
    type Error = Error;
    
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.peek()? {
            XmlEvent::StartElement { .. } => self.deserialize_map(visitor),
            _ => self.deserialize_string(visitor),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    serde_if_integer128! {
        fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.characters()?;
        let d: u8 = std::str::FromStr::from_str(&s).map_err(error::parse_int)?;
        visitor.visit_u8(d)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    serde_if_integer128! {
        fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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
        println!("  found characters {}", s);
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

    /// Parses `null` as None and any other values as `Some(...)`.
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    /// Parses a newtype struct as the underlying value.
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
        self.deserialize_seq(visitor)
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
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let map_value = visitor.visit_map(MapAccess::new(&mut self))?;
        Ok(map_value)
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
        println!("Struct {}", name);
        self.deserialize_map(visitor)
    }

    /// Parses an enum as a single key:value pair where the key identifies the
    /// variant and the value gives the content. A String will also parse correctly
    /// to a unit enum value.
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
                let tag_name = self.start_tag()?;
                self.tag_name = tag_name.clone();
                println!("Variant {}", tag_name);
                let v = visitor.visit_enum(VariantAccess::new(self))?;
                self.end_tag(tag_name)?;
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

struct VariantAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a + Read> VariantAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        VariantAccess { de: de }
    }
}

impl<'de, 'a, R: 'a + Read> de::EnumAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let v = seed.deserialize(self.de.current_tag()?.into_deserializer())?;
        Ok((v, self))
    }
}

impl<'de, 'a, R: 'a + Read> de::VariantAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        unimplemented!()
        //de::Deserialize::deserialize(self.de)
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        unimplemented!()
        //seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
        //de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_map(visitor)
    }
}

struct UnitVariantAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a + Read> UnitVariantAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        UnitVariantAccess { de: de }
    }
}

impl<'de, 'a, R: 'a + Read> de::EnumAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = try!(seed.deserialize(&mut *self.de));
        Ok((variant, self))
    }
}

impl<'de, 'a, R: 'a + Read> de::VariantAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(error::with_message("expected unit variant".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::from_str;
    
    #[test]
    fn one_element() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Document {
            value: String,
        }
        
        let expected = Document { value: "plain text".to_string() };
        
        let input = r"<document><value>plain text</value></document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn nested_elements() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Document {
            inner: InnerElement,
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        struct InnerElement {
            value: String,
        }
        
        let expected = Document { inner: InnerElement { value: "plain text".to_string() } };
        
        let input = r"
            <document>
                <inner>
                    <value>plain text</value>
                </inner>
            </document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn multiple_elements() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Document {
            first: String,
            second: String,
        }
        
        let expected = Document {
            first: "plain text".to_string(),
            second: "more text".to_string(),
        };
        
        let input = r"
            <document>
                <first>plain text</first>
                <second>more text</second>
            </document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    #[ignore]
    fn sequence() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Document {
            #[serde(rename = "item")]
            items: Vec<String>,
        }
        
        let expected = Document {
            items: vec!["first".to_string(), "second".to_string(), "third".to_string()],
        };
        
        let input = r"
            <document>
                <items>
                    <item>first</item>
                    <item>second</item>
                    <item>third</item>
                </items>
            </document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn unit_variant() {
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum ABC {
            A, B, C
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Document {
            content: ABC,
        }
        
        let expected = Document {
            content: ABC::A,
        };
        
        let input = r"
            <document>
                <content>a</content>
            </document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn struct_variant() {
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum Suit {
            CLUBS, DIAMONDS, HEARTS, SPADES,
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum Rank {
            ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, KNIGHT, QUEEN, KING
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum Card {
            Trump { number: u8 }, Fool, Suited { suit: Suit, rank: Rank },
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Document {
            content: Card,
        }
        
        let expected = Document {
            content: Card::Trump { number: 21 },
        };
        
        let input = r"
            <document>
                <content><trump><number>21</number></trump></content>
            </document>";
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
}
