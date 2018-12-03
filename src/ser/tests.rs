use super::to_string;

use ::tests::setup_logger;

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
fn option_absent() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Option<String>,
    }
    
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
fn option_present() {
    setup();
    
    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename = "document", rename_all = "kebab-case")]
    struct Document {
        content: Option<String>,
    }
    
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
