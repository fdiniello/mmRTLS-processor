use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use influxdb::Type;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MAC {
    s: [u8; 17],
}

impl MAC {
    pub fn new(s: &str) -> MAC {
        std::str::FromStr::from_str(s).unwrap()
    }
    pub fn as_str(&self) -> &str {
        let a = std::str::from_utf8(&self.s);
        a.unwrap()
    }
}

impl FromStr for MAC {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = MAC::default();
        m.s.copy_from_slice(s.as_bytes());
        Ok(m)
    }
}

impl Display for MAC {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.s))
    }
}

impl Debug for MAC {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.s))
    }
}

impl Into<Type> for MAC {
    fn into(self) -> Type {
        Type::Text(self.to_string())
    }
}

impl<'de> Deserialize<'de> for MAC {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MACVisitor {
            len: usize,
        }

        impl<'de> Visitor<'de> for MACVisitor {
            type Value = MAC;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a string containing at least {} bytes", self.len)
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.len() == self.len {
                    Ok(MAC::new(s))
                } else {
                    Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
                }
            }
        }

        let visitor = MACVisitor { len: 17 };
        deserializer.deserialize_str(visitor)
    }
}

impl Serialize for MAC {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
