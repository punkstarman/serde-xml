use std::collections::HashMap;

pub use super::{to_string, to_string_ns};

pub use crate::tests::setup_logger;

fn setup() {
    setup_logger();
}

#[test]
fn one_element() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        value: String,
    }

    let input = Document { value: "plain text".to_string() };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <value>plain text</value>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn nested_elements() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        inner: InnerElement,
    }

    #[derive(Debug, PartialEq, Serialize)]
    struct InnerElement {
        value: String,
    }

    let input = Document { inner: InnerElement { value: "plain text".to_string() } };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <inner>
            <value>plain text</value>
          </inner>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn multiple_elements() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        first: String,
        second: String,
    }

    let input = Document {
        first: "plain text".to_string(),
        second: "more text".to_string(),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <first>plain text</first>
          <second>more text</second>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn map() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: HashMap<String, String>
    };

    let input = Document {
        content: [
            ("second-key".to_string(), "more text".to_string()),
            ("first.key".to_string(), "plain text".to_string()),
        ].iter().cloned().collect(),
    };

    let expected1 = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>
            <first.key>plain text</first.key>
            <second-key>more text</second-key>
          </content>
        </document>"#);

    let expected2 = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>
            <second-key>more text</second-key>
            <first.key>plain text</first.key>
          </content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    trace!("Actual {:?}", actual);

    assert!(vec![expected1.to_string(), expected2.to_string()].contains(&actual));
}

#[test]
fn sequence() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        #[serde(rename = "item")]
        items: Vec<String>,
    }

    let input = Document {
        items: vec!["first".to_string(), "second".to_string(), "third".to_string()],
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <item>first</item>
          <item>second</item>
          <item>third</item>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn unit_variant() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum ABC {
        A, B, C
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: ABC,
    }

    let input = Document {
        content: ABC::A,
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>a</content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn struct_variant() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Suit {
        CLUBS, DIAMONDS, HEARTS, SPADES,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Rank {
        ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, KNIGHT, QUEEN, KING
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Card {
        Trump { number: u8 }, Fool, Suited { suit: Suit, rank: Rank },
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Card,
    }

    let input = Document {
        content: Card::Trump { number: 21 },
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>
            <trump>
              <number>21</number>
            </trump>
          </content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn newtype_variant() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Value {
        I(i64),
        F(f64),
        S(String),
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let input = Document {
        content: Value::I(42),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>
            <i>42</i>
          </content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn tuple_variant() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    #[allow(dead_code)]
    enum Value {
        I(i64),
        F(f64),
        S(String),
        Kv(String, String),
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let input = Document {
        content: Value::Kv("abc".to_string(), "123".to_string()),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>
            <kv>abc 123</kv>
          </content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn tuple_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value(String, String);

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let input = Document {
        content: Value("abc".to_string(), "123".to_string()),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>abc 123</content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn tuple() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: (i32, f64, String),
    }

    let input = Document {
        content: (123i32, 1.23f64, "abc".to_string()),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>123 1.23 abc</content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn types_unit() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: (),
    }

    let input = Document {
        content: (),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content />
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn unit_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value;

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let input = Document {
        content: Value,
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content />
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn newtype_struct() {
    setup();

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "kebab-case")]
    struct Value(String);

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Value,
    }

    let input = Document {
        content: Value("abc".to_string()),
    };

    let expected = indoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <document>
          <content>abc</content>
        </document>"#);

    let actual = to_string(&input).unwrap();

    assert_eq!(expected, actual);
}

mod option {
    use super::*;

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Option<String>,
    }

    #[test]
    fn absent() {
        setup();

        let input = Document {
            content: None,
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content />
            </document>"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn present() {
        setup();

        let input = Document {
            content: Some("123".to_string()),
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content>123</content>
            </document>"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }
}

mod attribute {
    use super::*;

    #[test]
    fn single() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@id")]
            id: String,
        }

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }

        let input = Document {
            content: Entity { id: "123".to_string() },
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content id="123" />
            </document>"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@x")]
            x: i32,
            #[serde(rename = "@y")]
            y: i32,
        }

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }

        let input = Document {
            content: Entity {
                x: 20,
                y: 40,
            },
        };

        let expected1 = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content x="20" y="40" />
            </document>"#);
        let expected2 = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content y="40" x="20" />
            </document>"#);

        let actual = to_string(&input).unwrap();

        trace!("{:?}", actual);

        assert!(vec![expected1.to_string(), expected2.to_string()].contains(&actual));
    }

    #[test]
    fn root() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
        }

        let input = Document {
            version: "1.2.3".to_string(),
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document version="1.2.3" />"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn root_and_subtag() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            #[serde(rename = "@version")]
            version: String,
            content: String,
        }

        let input = Document {
            version: "1.2.3".to_string(),
            content: "abc".to_string(),
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document version="1.2.3">
              <content>abc</content>
            </document>"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn attribute_and_body() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        struct Entity {
            #[serde(rename = "@id")]
            id: String,
            #[serde(rename = ".")]
            text: String,
        }

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: Entity,
        }

        let input = Document {
            content: Entity {
                id: "123".to_string(),
                text: "abc".to_string(),
            },
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document>
              <content id="123">abc</content>
            </document>"#);

        let actual = to_string(&input).unwrap();

        assert_eq!(expected, actual);
    }
}

mod ns {
    use super::*;

    #[test]
    fn root() {
        setup();

        #[derive(Debug, PartialEq, Serialize)]
        #[serde(rename = "document", rename_all = "kebab-case")]
        struct Document {
            content: String,
        }

        let input = Document {
            content: "abc 123".into(),
        };

        let expected = indoc!(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <document xmlns="urn:example:document">
              <content>abc 123</content>
            </document>"#);

        let actual = to_string_ns("urn:example:document", &input).unwrap();

        assert_eq!(expected, actual);
    }
}
