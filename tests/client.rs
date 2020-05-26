extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_xml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "document", rename_all = "kebab-case")]
struct Document {
    content: String,
}

#[test]
fn from_str() {
    let s = "<document><content>abc</content></document>";
    let _document: Document = serde_xml::from_str(s).unwrap();
}

#[test]
fn from_reader() {
    let s = "<document><content>abc</content></document>";
    let reader: Box<dyn std::io::Read> = Box::new(s.as_bytes());
    let _document: Document = serde_xml::from_reader(reader).unwrap();
}

#[test]
fn to_string() {
    let document = Document { content: "abc".to_string() };
    let _s: String = serde_xml::to_string(&document).unwrap();
}

#[test]
fn to_writer() {
    let document = Document { content: "abc".to_string() };
    let writer = Vec::with_capacity(128);
    serde_xml::to_writer(writer, &document).unwrap();
}
