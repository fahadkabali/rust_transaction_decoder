mod tests {
    use super::read_compact_size;
    #[test]
    fn test_read_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(7));

        let hex = "fd204e";
        let decoded = hex::decode(hex).unwrap();
        let mut bytes = decoded.as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 20_000_u64);
    }
}