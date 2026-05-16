use crate::utils::uuid::decode_uuid;
use crate::utils::uuid::encode_uuid;

#[test]
fn test_decode_uuid() {
    for _ in 0..600_000 {
        let id = uuid::Uuid::new_v4();
        let encoded = encode_uuid(&id);
        let decoded = decode_uuid(&encoded);
        assert_eq!(id, decoded);
    }
}