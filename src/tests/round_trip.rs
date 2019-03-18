use std::collections::HashMap;
use std::fmt::Debug;

pub use serde::{Serialize, Deserialize};

pub use ::ser::to_string;
pub use ::de::from_str;

pub use super::setup_logger;

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

    round_trip(&object);
}

#[test]
fn map() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: HashMap<String, String>
    };

    let object = Document {
        content: [
            ("first.key".to_string(), "plain text".to_string()),
            ("second-key".to_string(), "more text".to_string()),
        ].iter().cloned().collect()
    };

    round_trip(&object);
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
fn tuple_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value(String, String);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let object = Document {
        content: Value("abc".to_string(), "123".to_string()),
    };

    round_trip(&object);
}

#[test]
fn tuple() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: (i32, f64, String),
    }

    let object = Document {
        content: (123i32, 1.23f64, "abc".to_string()),
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

#[test]
fn types_unit() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: (),
    }

    let object = Document {
        content: (),
    };

    round_trip(&object);
}

#[test]
fn unit_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let object = Document {
        content: Value,
    };

    round_trip(&object);
}

#[test]
fn newtype_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value(String);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let object = Document {
        content: Value("abc".to_string()),
    };

    round_trip(&object);
}

mod option {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Option<String>,
    }

    #[test]
    fn absent() {
        setup();

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Option<String>,
        }

        let object = Document {
            content: None,
        };

        round_trip(&object);
    }

    #[test]
    fn present() {
        setup();

        let object = Document {
            content: Some("123".to_string()),
        };

        round_trip(&object);
    }
}

mod attribute {
    use super::*;

    #[test]
    fn single() {
        setup();

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@id")]
            id: String,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }

        let object = Document {
            content: Entity { id: "123".to_string() },
        };

        round_trip(&object);
    }

    #[test]
    fn root() {
        setup();

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
        }

        let object = Document {
            version: "1.2.3".to_string(),
        };

        round_trip(&object);
    }

    #[test]
    fn root_and_subtag() {
        setup();

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
            content: String,
        }

        let object = Document {
            version: "1.2.3".to_string(),
            content: "abc".to_string(),
        };

        round_trip(&object);
    }
}
