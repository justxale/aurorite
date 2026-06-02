use aurorite_util::uuid::{EncodedUuid, decode_uuid, encode_uuid};

#[test]
fn test_decode_uuid() {
    for _ in 0..600_000 {
        let id = uuid::Uuid::new_v4();
        let encoded = encode_uuid(&id);
        let decoded = decode_uuid(&encoded);
        assert_eq!(id, decoded);
    }
}

#[test]
fn test_serde_support() -> Result<(), serde_json::Error> {
    for _ in 0..600_000 {
        let id = EncodedUuid::now_v7();
        let deserialized = serde_json::to_string(&id)?;
        let id2 = serde_json::from_str::<EncodedUuid>(&deserialized)?;
        assert_eq!(id, id2);
    }
    Ok(())
}
