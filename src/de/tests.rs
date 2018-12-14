use std::collections::HashMap;

pub use super::from_str;

pub use ::tests::setup_logger;

fn setup() {
    setup_logger();
}

#[test]
fn one_element() {
    setup();
    
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
    setup();
    
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
    setup();
    
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
fn map() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document(HashMap<String, String>);
    
    let expected = Document([
        ("first.key".to_string(), "plain text".to_string()),
        ("second-key".to_string(), "more text".to_string()),
        ].iter().cloned().collect());
    
    let input = r"
        <document>
            <first.key>plain text</first.key>
            <second-key>more text</second-key>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn sequence() {
    setup();
    
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
            <item>first</item>
            <item>second</item>
            <item>third</item>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn unit_variant() {
    setup();
    
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
    setup();
    
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

#[test]
fn newtype_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Value {
        I(i64),
        F(f64),
        S(String),
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value::I(42),
    };
    
    let input = r#"
        <document>
            <content>
                <i>42</i>
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn tuple_variant() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Value {
        I(i64),
        F(f64),
        S(String),
        Kv(String, String),
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value::Kv("abc".to_string(), "123".to_string()),
    };
    
    let input = r#"
        <document>
            <content>
                <kv>abc 123</kv>
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn tuple_struct() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Value(String, String);
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value("abc".to_string(), "123".to_string()),
    };
    
    let input = r#"
        <document>
            <content>
                abc 123
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn tuple() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: (i32, f64, String),
    }
    
    let expected = Document {
        content: (123i32, 1.23f64, "abc".to_string()),
    };
    
    let input = r#"
        <document>
            <content>
                123 1.23
                abc
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn types_char() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: char,
    }
    
    let expected = Document {
        content: 'a',
    };
    
    let input = r#"<document><content>a</content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn types_float() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: f64,
    }
    
    let expected = Document {
        content: 1.25f64,
    };
    
    let input = r#"<document><content>1.25</content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn types_bool() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: bool,
    }
    
    let expected = Document {
        content: true,
    };
    
    let input = r#"<document><content>true</content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn types_unit() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: (),
    }
    
    let expected = Document {
        content: (),
    };
    
    let input = r#"<document><content></content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn unit_struct() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value;
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value,
    };
    
    let input = r#"<document><content></content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn newtype_struct() {
    setup();
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value(String);
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value("abc".to_string()),
    };
    
    let input = r#"<document><content>abc</content></document>"#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

mod option {
    use super::*;
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Document {
        content: Option<String>,
    }
    
    #[test]
    fn empty() {
        setup();
        
        let expected = Document {
            content: None,
        };
        
        let input = r#"<document><content></content></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn absent() {
        setup();
        
        let expected = Document {
            content: None,
        };
        
        let input = r#"<document></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn present() {
        setup();
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Document {
            content: Option<String>,
        }
        
        let expected = Document {
            content: Some("abc".to_string()),
        };
        
        let input = r#"<document><content>abc</content></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
}

mod attribute {
    use super::*;
    
    #[test]
    fn single() {
        setup();
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@id")]
            id: String,
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }
        
        let expected = Document {
            content: Entity { id: "123".to_string() },
        };
        
        let input = r#"<document><content id="123" /></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn root() {
        setup();
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
        }
        
        let expected = Document {
            version: "1.2.3".to_string(),
        };
        
        let input = r#"<document version="1.2.3"></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn root_and_subtag() {
        setup();
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
            content: String,
        }
        
        let expected = Document {
            version: "1.2.3".to_string(),
            content: "abc".to_string(),
        };
        
        let input = r#"<document version="1.2.3"><content>abc</content></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn attribute_and_body() {
        setup();
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@id")]
            id: String,
            #[serde(rename = ".")]
            text: String,
        }
        
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }
        
        let expected = Document {
            content: Entity {
                id: "123".to_string(),
                text: "abc".to_string(),
            },
        };
        
        let input = r#"<document><content id="123">abc</content></document>"#;
        
        let actual: Document = from_str(input).unwrap();
        
        assert_eq!(expected, actual);
    }
}
