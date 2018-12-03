use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use ::ser::to_string;
use ::de::from_str;

use super::setup_logger;

fn setup() {
    setup_logger();
}

fn round_trip<T>(object: &T) -> ()
where
    T: Debug + PartialEq + Serialize + for<'de> Deserialize<'de>
{
    let actual_repr = to_string(object).unwrap();
    debug!("actual: {}", actual_repr);
    let actual: T = from_str(&actual_repr).unwrap();
    
    assert_eq!(object, &actual);
}

#[test]
fn one_element() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        value: String,
    }
    
    let object = Document { value: "plain text".to_string() };
    
    round_trip(&object);
}

#[test]
fn nested_elements() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        inner: InnerElement,
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct InnerElement {
        value: String,
    }
    
    let object = Document { inner: InnerElement { value: "plain text".to_string() } };
    
    round_trip(&object);
}

#[test]
fn multiple_elements() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        first: String,
        second: String,
    }
    
    let object = Document {
        first: "plain text".to_string(),
        second: "more text".to_string(),
    };
    
    round_trip(&object)
}

#[test]
fn sequence() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        #[serde(rename = "item")]
        items: Vec<String>,
    }
    
    let object = Document {
        items: vec!["first".to_string(), "second".to_string(), "third".to_string()],
    };
    
    round_trip(&object);
}

#[test]
fn unit_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum ABC {
        A, B, C
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: ABC,
    }
    
    let object = Document {
        content: ABC::A,
    };
    
    round_trip(&object);
}

#[test]
fn struct_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Suit {
        CLUBS, DIAMONDS, HEARTS, SPADES,
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Rank {
        ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, KNIGHT, QUEEN, KING
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Card {
        Trump { number: u8 }, Fool, Suited { suit: Suit, rank: Rank },
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Card,
    }
    
    let object = Document {
        content: Card::Trump { number: 21 },
    };
    
    round_trip(&object);
}

#[test]
fn newtype_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Value {
        I(i64),
        F(f64),
        S(String),
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }
    
    let object = Document {
        content: Value::I(42),
    };
    
    round_trip(&object);
}

#[test]
fn tuple_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Value {
        I(i64),
        F(f64),
        S(String),
        Kv(String, String),
    }
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }
    
    let object = Document {
        content: Value::Kv("abc".to_string(), "123".to_string()),
    };
    
    round_trip(&object);
}

#[test]
fn types_bool() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: bool,
    }
    
    let object = Document {
        content: true,
    };
    
    round_trip(&object);
}