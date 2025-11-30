use std::fmt::Debug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct Id {
    val: u64
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.serialize_u64(self.val)
    }
}

impl Id {
    pub fn new() -> Id {
        Id { val: 1 }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.val)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u64::deserialize(deserializer)?;
        Ok(Id { val })
    }
}
