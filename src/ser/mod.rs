use serde::ser;

use super::error::Result;

pub struct Serializer;

pub fn to_string<T: ?Sized>(_value: &T) -> Result<String>
where
    T: ser::Serialize,
{
    unimplemented!()
}