use serde::Deserialize;
use std::iter::Iterator;

const ALPHABET_LEN: u128 = 64;
pub const ALPHABET: [char; ALPHABET_LEN as usize] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V','W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
    '_', '-'
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

pub fn decode_uuid(encoded: &String) -> uuid::Uuid {
    let mut uuid_int: u128 = 0;
    for (i, digit) in encoded.chars().rev().enumerate() {
        let tmp = get_number(digit) as u128;
        uuid_int += tmp * ALPHABET_LEN.pow(i as u32) as u128
    }

    uuid::Uuid::from_u128(uuid_int)
}

pub fn encode_uuid(uuid: &uuid::Uuid) -> String {
    let mut n = uuid.as_u128();
    let mut res = String::with_capacity(21);
    while n > 0 {
        let c = ALPHABET[(n % ALPHABET_LEN as u128) as usize];
        res.push(c);
        n /= ALPHABET_LEN as u128;
    }
    res.chars().rev().collect()
}

pub fn serialize_encoded_uuid<S>(uuid: &uuid::Uuid, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.collect_str(&encode_uuid(uuid))
}

pub fn deserialize_encoded_uuid<'de, D>(des: D) -> Result<uuid::Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(decode_uuid(&String::deserialize(des)?))
}

#[cfg(test)]
mod tests {
    use crate::utils::uuid::encode_uuid;
    use crate::utils::uuid::decode_uuid;

    #[test]
    fn test_decode_uuid() {
        for _ in 0..600_000 {
            let id = uuid::Uuid::new_v4();
            let encoded = encode_uuid(&id);
            let decoded = decode_uuid(&encoded);
            assert_eq!(id, decoded);
        }
    }
}
