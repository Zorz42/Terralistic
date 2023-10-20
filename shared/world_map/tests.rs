#[cfg(test)]
mod tests {
    use crate::shared::world_map::world_map::WorldMap;

    #[test]
    fn test_translate_coords() {
        let map = WorldMap::new(10, 10);

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
        assert_eq!(map.get_width(), 0);
        assert_eq!(map.get_height(), 0);
        map.translate_coords(0, 0).unwrap_err();
    }

    #[test]
    fn test_deserialize_serialize() {
        let map = WorldMap::new(10, 10);
        let serialized = serde_json::to_string(&map).unwrap();
        let deserialized: WorldMap = serde_json::from_str(&serialized).unwrap();
        assert_eq!(map.get_width(), deserialized.get_width());
        assert_eq!(map.get_height(), deserialized.get_height());
    }
}
