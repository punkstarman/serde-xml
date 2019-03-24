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
    #[serde(rename = "schema", rename_all = "camelCase")]
    struct XmlSchema {
        #[serde(rename = "@targetNamespace")]
        target_namespace: String,
    }

    #[test]
    fn read() {
        setup();

        let file = File::open("src/tests/XMLSchema.xsd").unwrap();
        let _: XmlSchema = from_reader(file).unwrap();
    }
}
