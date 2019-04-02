pub use std::fs::File;

pub use serde::{Serialize, Deserialize};

pub use ::ser::to_string;
pub use ::de::from_reader;

pub use ::decoration::choice_seq;

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
    #[serde(rename_all = "camelCase")]
    struct Import {
        #[serde(rename = "@id")]
        id: Option<String>,
        #[serde(rename = "@namespace")]
        namespace: String,
        #[serde(rename = "@schemaLocation")]
        schema_location: String,
        annotation: Option<Annotation>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum SchemaDecl {
        Annotation(Annotation),
        Import(Import),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "schema", rename_all = "camelCase")]
    struct XmlSchema {
        #[serde(rename = "@targetNamespace")]
        target_namespace: String,
        #[serde(rename = "@version")]
        version: String,
        #[serde(rename = "@finalDefault")]
        final_default: Option<String>,
        #[serde(rename = "@blockDefault")]
        block_default: Option<String>,
        #[serde(rename = "@attributeFormDefault")]
        attribute_form_default: Option<String>,
        #[serde(rename = "@elementFormDefault")]
        element_form_default: Option<String>,
        #[serde(rename = "@id")]
        id: Option<String>,
        #[serde(rename = "@lang")]
        lang: Option<String>,
        #[serde(flatten, with = "choice_seq")]
        decls: Vec<SchemaDecl>,
    }

    #[test]
    fn read() {
        setup();

        let file = File::open("src/tests/XMLSchema.xsd").unwrap();
        let schema: XmlSchema = from_reader(file).unwrap();

        assert_eq!(schema.target_namespace, "http://www.w3.org/2001/XMLSchema");
        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.decls.len(), 4);
        // assert_eq!(schema.annotations.len(), 3);
        // assert_eq!(schema.imports.len(), 1);
        // assert_eq!(schema.imports[0].namespace, "http://www.w3.org/XML/1998/namespace");
        // assert!(schema.imports[0].annotation.is_some());
    }
}
