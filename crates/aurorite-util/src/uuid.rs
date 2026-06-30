use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::str::FromStr;
use uuid::Uuid;

const ALPHABET_LEN: u128 = 64;
pub const ALPHABET: [char; ALPHABET_LEN as usize] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '_', '-',
];

fn get_number(digit: char) -> u8 {
    if digit.is_ascii_digit() {
        (digit as u8) - b'0'
    } else if digit.is_ascii_uppercase() {
        (digit as u8) - b'A' + 10
    } else if digit.is_ascii_lowercase() {
        (digit as u8) - b'a' + 36
    } else if digit == '_' {
        62
    } else {
        63
    }
}

pub fn decode_uuid(encoded: &str) -> Uuid {
    let mut uuid_int: u128 = 0;
    for (i, digit) in encoded.chars().rev().enumerate() {
        let tmp = get_number(digit) as u128;
        uuid_int += tmp * ALPHABET_LEN.pow(i as u32)
    }

    Uuid::from_u128(uuid_int)
}

pub fn encode_uuid(uuid: &Uuid) -> String {
    let mut n = uuid.as_u128();
    let mut res = String::with_capacity(21);
    while n > 0 {
        let c = ALPHABET[(n % ALPHABET_LEN) as usize];
        res.push(c);
        n /= ALPHABET_LEN;
    }
    res.chars().rev().collect()
}

pub fn serialize_encoded_uuid<S>(uuid: &Uuid, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.collect_str(&encode_uuid(uuid))
}

pub fn deserialize_encoded_uuid<'de, D>(des: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(decode_uuid(&String::deserialize(des)?))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EncodedUuid(pub Uuid);

impl EncodedUuid {
    pub fn now_v7() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl FromStr for EncodedUuid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EncodedUuid(decode_uuid(s)))
    }
}

impl Display for EncodedUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", encode_uuid(&self.0)))
    }
}

impl Serialize for EncodedUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{}", self))
    }
}

struct EncodedUuidVisitor;

impl<'vi> Visitor<'vi> for EncodedUuidVisitor {
    type Value = EncodedUuid;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expected encoded uuid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(EncodedUuid(decode_uuid(v)))
    }
}

impl<'de> Deserialize<'de> for EncodedUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EncodedUuidVisitor)
    }
}

pub mod serde_support {
    use super::{deserialize_encoded_uuid, serialize_encoded_uuid};
    use serde::{Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Uuid, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_encoded_uuid(uuid, ser)
    }

    pub fn deserialize<'de, D>(des: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_encoded_uuid(des)
    }
}

#[cfg(test)]
mod tests {
    use crate::uuid::{decode_uuid, encode_uuid, EncodedUuid};
    const LOOPS: usize = 300_000;

    #[test]
    fn test_decode_uuid() {
        for _ in 0..LOOPS {
            let id = uuid::Uuid::now_v7();
            let encoded = encode_uuid(&id);
            let decoded = decode_uuid(&encoded);
            assert_eq!(id, decoded);
        }
    }

    #[test]
    fn test_serde_support() -> Result<(), serde_json::Error> {
        for _ in 0..LOOPS {
            let id = EncodedUuid::now_v7();
            let deserialized = serde_json::to_string(&id)?;
            let id2 = serde_json::from_str::<EncodedUuid>(&deserialized)?;
            assert_eq!(id, id2);
        }
        Ok(())
    }
}