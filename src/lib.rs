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

mod de;
mod error;
mod ser;

pub use self::de::{from_str, from_reader, Deserializer};
pub use self::error::{Error, Result};
pub use self::ser::{to_string, to_writer, Serializer};

#[cfg(test)]
mod tests;
