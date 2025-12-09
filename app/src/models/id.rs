use std::{fmt::{Debug, Display}};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct Id {
    id: u64
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.serialize_u64(self.id)
    }
}

impl Display for Id {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.serialize_u64(self.id)
    }
}

impl Id {
    pub fn new(num: u64) -> Id {
        Id { id: num }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.id)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u64::deserialize(deserializer)?;
        Ok(Id { id: val })
    }
}

impl PartialEq<u64> for Id {
    fn eq(&self, other: &u64) -> bool {
        self.id == *other
    }
}
