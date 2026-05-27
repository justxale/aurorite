use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
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
        (digit as u8) - ('0' as u8)
    } else if digit.is_ascii_uppercase() {
        (digit as u8) - ('A' as u8) + 10
    } else if digit.is_ascii_lowercase() {
        (digit as u8) - ('a' as u8) + 36
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

    pub fn from_str(encoded: &str) -> Self {
        EncodedUuid(decode_uuid(encoded))
    }

    pub fn uuid(&self) -> Uuid {
        self.0
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
        serializer.collect_str(&format_args!("{}", self.to_string()))
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
