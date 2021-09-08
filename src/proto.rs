tonic::include_proto!("ble_ws");

impl From<uuid::Uuid> for Uuid {
    fn from(other: uuid::Uuid) -> Self {
        let mut n = other.as_u128();
        let time_low = n as u32;
        n >>= 32;
        let time_mid = n as u16;
        n >>= 16;
        let time_hi_and_version = n as u16;
        n >>= 16;
        let clock_seq_hi_and_version = n as u8;
        n >>= 8;
        let clock_seq_lo = n as u8;
        n >>= 8;
        let node = n as u64;
        Uuid {
            time_low,
            time_mid: time_mid.into(),
            time_hi_and_version: time_hi_and_version.into(),
            clock_seq_hi_and_reserved: clock_seq_hi_and_version.into(),
            clock_seq_low: clock_seq_lo.into(),
            node,
        }
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(other: Uuid) -> Self {
        let mut n = 0_u128;
        n |= u128::from(other.node);
        n <<= 8;
        n |= u128::from(other.clock_seq_low);
        n <<= 8;
        n |= u128::from(other.clock_seq_hi_and_reserved);
        n <<= 16;
        n |= u128::from(other.time_hi_and_version);
        n <<= 16;
        n |= u128::from(other.time_mid);
        n <<= 32;
        n |= u128::from(other.time_low);
        uuid::Uuid::from_u128(n)
    }
}

#[test]
fn uuid_roundtrip() {
    let original_uuid = uuid::Uuid::parse_str("0abb8598-5c9f-4907-9409-3af2474efe07").unwrap();
    let proto = Uuid::from(original_uuid);
    let converted = uuid::Uuid::from(proto);

    assert_eq!(original_uuid, converted);
}
