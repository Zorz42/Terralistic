#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::shared::world_map::world_map::WorldMap;
    use crate::shared::world_map::CHUNK_SIZE;

    #[test]
    fn test_translate_coords() {
        let map = WorldMap::new((10, 10));

        assert_eq!(map.translate_coords(0, 0).unwrap(), 0);
        assert_eq!(map.translate_coords(9, 9).unwrap(), 99);
        assert_eq!(map.translate_coords(5, 5).unwrap(), 55);
        map.translate_coords(10, 10).unwrap_err();
        map.translate_coords(-1, -1).unwrap_err();
        map.translate_coords(0, -1).unwrap_err();
        map.translate_coords(-1, 0).unwrap_err();
        map.translate_coords(10, 0).unwrap_err();
        map.translate_coords(0, 10).unwrap_err();
        map.translate_coords(1234, 1234).unwrap_err();
    }

    #[test]
    fn test_new_empty() {
        let map = WorldMap::new_empty();
        assert_eq!(map.get_size(), (0, 0));
        map.translate_coords(0, 0).unwrap_err();
    }

    #[test]
    fn test_deserialize_serialize() {
        let map = WorldMap::new((10, 10));
        let serialized = serde_json::to_string(&map).unwrap();
        let deserialized: WorldMap = serde_json::from_str(&serialized).unwrap();
        assert_eq!(map.get_size(), deserialized.get_size());
    }

    /// The two coordinate schemes deliberately disagree: `translate_coords` is
    /// column major (`x * height + y`) while `translate_chunk_coords` is row major
    /// (`x + y * width`). Both are internally consistent, and this pins that down so
    /// nobody "fixes" one of them in isolation.
    #[test]
    fn test_the_two_schemes_have_different_orderings() {
        let map = WorldMap::new((10, 20));

        // moving in y is a step of 1 for blocks
        assert_eq!(map.translate_coords(0, 1).unwrap() - map.translate_coords(0, 0).unwrap(), 1);
        // but moving in x is a step of 1 for chunks
        let map = WorldMap::new((10 * CHUNK_SIZE as u32, 20 * CHUNK_SIZE as u32));
        assert_eq!(map.translate_chunk_coords(1, 0).unwrap() - map.translate_chunk_coords(0, 0).unwrap(), 1);
    }

    #[test]
    fn test_translate_chunk_coords() {
        // 4 x 3 chunks
        let map = WorldMap::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32));

        assert_eq!(map.translate_chunk_coords(0, 0).unwrap(), 0);
        assert_eq!(map.translate_chunk_coords(3, 2).unwrap(), 11);
        assert_eq!(map.translate_chunk_coords(0, 1).unwrap(), 4);
    }

    #[test]
    fn test_translate_chunk_coords_out_of_bounds() {
        let map = WorldMap::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32));

        map.translate_chunk_coords(4, 0).unwrap_err();
        map.translate_chunk_coords(0, 3).unwrap_err();
        map.translate_chunk_coords(-1, 0).unwrap_err();
        map.translate_chunk_coords(0, -1).unwrap_err();
    }

    /// Every chunk index is distinct and inside the chunk grid.
    #[test]
    fn test_chunk_indices_are_unique() {
        let map = WorldMap::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32));

        let mut seen = std::collections::HashSet::new();
        for x in 0..4 {
            for y in 0..3 {
                let index = map.translate_chunk_coords(x, y).unwrap();
                assert!(index < 12, "chunk index {index} is past the end of the grid");
                assert!(seen.insert(index), "chunk index {index} was produced twice");
            }
        }
    }

    /// A world whose size is not a whole number of chunks truncates: the partial chunk on
    /// the edge is not addressable.
    #[test]
    fn test_partial_chunks_are_not_addressable() {
        let map = WorldMap::new((CHUNK_SIZE as u32 * 2 + 5, CHUNK_SIZE as u32 * 2));

        map.translate_chunk_coords(1, 0).unwrap();
        assert!(map.translate_chunk_coords(2, 0).is_err(), "the partial edge chunk should be out of range");
    }

    #[test]
    fn test_block_indices_are_unique() {
        let map = WorldMap::new((7, 5));

        let mut seen = std::collections::HashSet::new();
        for x in 0..7 {
            for y in 0..5 {
                let index = map.translate_coords(x, y).unwrap();
                assert!(index < 35);
                assert!(seen.insert(index), "index {index} produced twice");
            }
        }
    }
}
