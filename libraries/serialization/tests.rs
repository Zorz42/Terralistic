#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::serialization;
    use std::collections::HashMap;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Debug)]
    struct Sample {
        name: String,
        numbers: Vec<i32>,
        flag: bool,
        maybe: Option<u8>,
    }

    #[test]
    fn test_round_trip_primitives() {
        for value in [0_i32, 1, -1, i32::MAX, i32::MIN] {
            let bytes = serialization::serialize(&value).unwrap();
            assert_eq!(serialization::deserialize::<i32>(&bytes).unwrap(), value);
        }
    }

    #[test]
    fn test_round_trip_struct() {
        let sample = Sample {
            name: "terralistic".to_owned(),
            numbers: vec![1, 2, 3, -4],
            flag: true,
            maybe: Some(9),
        };

        let bytes = serialization::serialize(&sample).unwrap();
        assert_eq!(serialization::deserialize::<Sample>(&bytes).unwrap(), sample);
    }

    #[test]
    fn test_round_trip_collections() {
        let mut map = HashMap::new();
        map.insert("a".to_owned(), vec![1_u8, 2, 3]);
        map.insert("b".to_owned(), vec![]);

        let bytes = serialization::serialize(&map).unwrap();
        assert_eq!(serialization::deserialize::<HashMap<String, Vec<u8>>>(&bytes).unwrap(), map);
    }

    #[test]
    fn test_round_trip_empty_and_none() {
        let empty: Vec<i32> = Vec::new();
        let bytes = serialization::serialize(&empty).unwrap();
        assert_eq!(serialization::deserialize::<Vec<i32>>(&bytes).unwrap(), empty);

        let none: Option<u32> = None;
        let bytes = serialization::serialize(&none).unwrap();
        assert_eq!(serialization::deserialize::<Option<u32>>(&bytes).unwrap(), none);
    }

    #[test]
    fn test_serialize_into_matches_serialize() {
        let sample = vec![1_u32, 2, 3];

        let direct = serialization::serialize(&sample).unwrap();
        let mut buffer = Vec::new();
        serialization::serialize_into(&mut buffer, &sample).unwrap();

        assert_eq!(direct, buffer, "serialize and serialize_into should produce the same bytes");
    }

    #[test]
    fn test_deserialize_rejects_truncated_input() {
        let bytes = serialization::serialize(&"a reasonably long string".to_owned()).unwrap();
        let truncated = bytes.get(..bytes.len() / 2).unwrap();

        serialization::deserialize::<String>(truncated).unwrap_err();
    }

    /// The format uses variable length integers, so small numbers are cheap. This is the
    /// property that made the world save shrink when moving off bincode 1's fixed width integers.
    #[test]
    fn test_small_integers_are_compact() {
        let small = serialization::serialize(&1_u64).unwrap();
        let large = serialization::serialize(&u64::MAX).unwrap();

        assert!(small.len() < large.len(), "varint encoding should make small numbers shorter");
    }
}
