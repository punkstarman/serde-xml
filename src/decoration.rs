
pub mod choice_seq {
    use std::fmt::Debug;
    use std::marker::PhantomData;
    use std::fmt;
    use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
    use serde::ser::{Serialize, Serializer};

    struct VecEnumVisitor<T>(PhantomData<Vec<T>>);

    impl<'de, T> Visitor<'de> for VecEnumVisitor<T>
    where
        T: Deserialize<'de> + Debug,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a vec of enum")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Vec<T>, A::Error>
        where
            A: MapAccess<'de>,
        {
            trace!("magic happens here");
            let mut values = Vec::new();
            
            while let Some((_, value)) = access.next_entry::<String, T>()? {
                trace!("magic {:?}", value);
                values.push(value);
            }
            Ok(values)
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: Deserialize<'de> + Debug,
        D: Deserializer<'de>,
    {
        trace!("dragons!");
        deserializer.deserialize_map(VecEnumVisitor(PhantomData))
    }

    pub fn serialize<T, S>(data: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize + Debug,
        S: Serializer,
    {
        let _ = data;
        let _ = serializer;
        trace!("marker");
        unimplemented!()
    }
}
