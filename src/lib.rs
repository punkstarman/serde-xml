#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

extern crate xml;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate indoc;

#[cfg(test)]
extern crate env_logger;

pub mod decoration;

mod de;
mod error;
mod ser;

pub use de::{from_str, from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, to_writer, Serializer};

#[cfg(test)]
mod tests;
