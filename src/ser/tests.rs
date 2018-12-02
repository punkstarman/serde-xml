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