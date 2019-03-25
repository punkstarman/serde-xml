pub use std::fs::File;

pub use serde::{Serialize, Deserialize};

pub use ::ser::to_string;
pub use ::de::from_reader;

pub use super::setup_logger;

fn setup() {
    setup_logger();
}

mod xsd {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Documentation {
        #[serde(rename = "@source")]
        source: Option<String>,
        #[serde(rename = ".")]
        content: String,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Annotation {
        #[serde(rename = "@id")]
        id: Option<String>,
        #[serde(rename = "documentation")]
        documentations: Vec<Documentation>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "schema", rename_all = "camelCase")]
    struct XmlSchema {
        #[serde(rename = "@targetNamespace")]
        target_namespace: String,
        #[serde(rename = "@version")]
        version: String,
        #[serde(rename = "annotation")]
        annotations: Vec<Annotation>,
    }

    #[test]
    fn read() {
        setup();

        let file = File::open("src/tests/XMLSchema.xsd").unwrap();
        let schema: XmlSchema = from_reader(file).unwrap();

        assert_eq!(schema.target_namespace, "http://www.w3.org/2001/XMLSchema");
        assert_eq!(schema.version, "1.0");
    }
}
