extern crate env_logger;

use super::to_string;

fn setup() {
    setup_logger();
}

fn setup_logger() {
    let _ = env_logger::try_init();
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
    struct Document {
        #[serde(rename = "item")]
        items: Vec<String>,
    }
    
    let input = Document {
        items: vec!["first".to_string(), "second".to_string(), "third".to_string()],
    };
    
    let expected = indoc!(r#"
        <document>
            <item>first</item>
            <item>second</item>
            <item>third</item>
        </document>"#);
    
    let actual = to_string(&input).unwrap();
    
    assert_eq!(expected, actual);
}
