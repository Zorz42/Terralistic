#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::libraries::grid::{ChunkTracker, Chunks, Grid};

    const CHUNK_SIZE: i32 = 16;

    fn grid_of(size: (u32, u32)) -> Grid<i32> {
        Grid::filled(size, 0)
    }

    // --- Grid ---

    #[test]
    fn test_translate_coords() {
        let grid = grid_of((10, 10));

        assert_eq!(grid.translate_coords(0, 0).unwrap(), 0);
        assert_eq!(grid.translate_coords(9, 9).unwrap(), 99);
        assert_eq!(grid.translate_coords(5, 5).unwrap(), 55);
        grid.translate_coords(10, 10).unwrap_err();
        grid.translate_coords(-1, -1).unwrap_err();
        grid.translate_coords(0, -1).unwrap_err();
        grid.translate_coords(-1, 0).unwrap_err();
        grid.translate_coords(10, 0).unwrap_err();
        grid.translate_coords(0, 10).unwrap_err();
        grid.translate_coords(1234, 1234).unwrap_err();
    }

    #[test]
    fn test_new_empty() {
        let grid = Grid::<i32>::new_empty();
        assert_eq!(grid.get_size(), (0, 0));
        grid.translate_coords(0, 0).unwrap_err();
        grid.get(0, 0).unwrap_err();
    }

    #[test]
    fn test_filled_holds_the_fill_value_everywhere() {
        let grid = Grid::filled((3, 4), 7);

        assert_eq!(grid.get_size(), (3, 4));
        assert_eq!(grid.cells().len(), 12);
        for x in 0..3 {
            for y in 0..4 {
                assert_eq!(*grid.get(x, y).unwrap(), 7);
            }
        }
    }

    #[test]
    fn test_get_and_set() {
        let mut grid = grid_of((4, 4));

        grid.set(2, 3, 42).unwrap();
        assert_eq!(*grid.get(2, 3).unwrap(), 42);
        assert_eq!(*grid.get(3, 2).unwrap(), 0, "setting (2, 3) must not touch (3, 2)");

        *grid.get_mut(0, 0).unwrap() = 1;
        assert_eq!(*grid.get(0, 0).unwrap(), 1);
    }

    /// Every access is checked, so a coordinate outside the grid is an error rather than a
    /// read of whichever cell the arithmetic happened to land on.
    #[test]
    fn test_out_of_bounds_access_is_an_error() {
        let mut grid = grid_of((4, 4));

        grid.get(4, 0).unwrap_err();
        grid.get(0, -1).unwrap_err();
        grid.get_mut(4, 4).unwrap_err();
        grid.set(-1, 0, 1).unwrap_err();
    }

    #[test]
    fn test_contains() {
        let grid = grid_of((4, 4));

        assert!(grid.contains(0, 0));
        assert!(grid.contains(3, 3));
        assert!(!grid.contains(4, 3));
        assert!(!grid.contains(-1, 0));
    }

    #[test]
    fn test_from_columns() {
        let grid = Grid::from_columns(&[vec![1, 2, 3], vec![4, 5, 6]]).unwrap();

        assert_eq!(grid.get_size(), (2, 3), "the outer slice is x, the inner is y");
        assert_eq!(*grid.get(0, 0).unwrap(), 1);
        assert_eq!(*grid.get(0, 2).unwrap(), 3);
        assert_eq!(*grid.get(1, 0).unwrap(), 4);
        assert_eq!(*grid.get(1, 2).unwrap(), 6);
    }

    #[test]
    fn test_from_columns_rejects_empty_and_ragged() {
        Grid::<i32>::from_columns(&[]).unwrap_err();
        Grid::from_columns(&[vec![1, 2], vec![3]]).unwrap_err();
    }

    #[test]
    fn test_deserialize_serialize() {
        let mut grid = grid_of((10, 10));
        grid.set(4, 7, 99).unwrap();

        let serialized = serde_json::to_string(&grid).unwrap();
        let deserialized: Grid<i32> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(grid.get_size(), deserialized.get_size());
        assert_eq!(*deserialized.get(4, 7).unwrap(), 99);
    }

    #[test]
    fn test_cell_indices_are_unique() {
        let grid = grid_of((7, 5));

        let mut seen = std::collections::HashSet::new();
        for x in 0..7 {
            for y in 0..5 {
                let index = grid.translate_coords(x, y).unwrap();
                assert!(index < 35);
                assert!(seen.insert(index), "index {index} produced twice");
            }
        }
    }

    // --- Chunks ---

    /// The two coordinate schemes deliberately disagree: `Grid` is column major
    /// (`x * height + y`) while `Chunks` is row major (`x + y * width`). Both are
    /// internally consistent, and this pins that down so nobody "fixes" one in isolation.
    #[test]
    fn test_the_two_schemes_have_different_orderings() {
        let grid = grid_of((10, 20));
        // moving in y is a step of 1 for cells
        assert_eq!(grid.translate_coords(0, 1).unwrap() - grid.translate_coords(0, 0).unwrap(), 1);

        // but moving in x is a step of 1 for chunks
        let chunks = Chunks::new((10 * CHUNK_SIZE as u32, 20 * CHUNK_SIZE as u32), CHUNK_SIZE);
        assert_eq!(chunks.translate_coords(1, 0).unwrap() - chunks.translate_coords(0, 0).unwrap(), 1);
    }

    #[test]
    fn test_translate_chunk_coords() {
        // 4 x 3 chunks
        let chunks = Chunks::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32), CHUNK_SIZE);

        assert_eq!(chunks.get_size(), (4, 3));
        assert_eq!(chunks.count(), 12);
        assert_eq!(chunks.translate_coords(0, 0).unwrap(), 0);
        assert_eq!(chunks.translate_coords(3, 2).unwrap(), 11);
        assert_eq!(chunks.translate_coords(0, 1).unwrap(), 4);
    }

    #[test]
    fn test_translate_chunk_coords_out_of_bounds() {
        let chunks = Chunks::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32), CHUNK_SIZE);

        chunks.translate_coords(4, 0).unwrap_err();
        chunks.translate_coords(0, 3).unwrap_err();
        chunks.translate_coords(-1, 0).unwrap_err();
        chunks.translate_coords(0, -1).unwrap_err();
    }

    /// Every chunk index is distinct and inside the chunk grid.
    #[test]
    fn test_chunk_indices_are_unique() {
        let chunks = Chunks::new((4 * CHUNK_SIZE as u32, 3 * CHUNK_SIZE as u32), CHUNK_SIZE);

        let mut seen = std::collections::HashSet::new();
        for x in 0..4 {
            for y in 0..3 {
                let index = chunks.translate_coords(x, y).unwrap();
                assert!(index < 12, "chunk index {index} is past the end of the grid");
                assert!(seen.insert(index), "chunk index {index} was produced twice");
            }
        }
    }

    /// A grid whose size is not a whole number of chunks truncates: the partial chunk on
    /// the edge is not addressable.
    #[test]
    fn test_partial_chunks_are_not_addressable() {
        let chunks = Chunks::new((CHUNK_SIZE as u32 * 2 + 5, CHUNK_SIZE as u32 * 2), CHUNK_SIZE);

        chunks.translate_coords(1, 0).unwrap();
        assert!(chunks.translate_coords(2, 0).is_err(), "the partial edge chunk should be out of range");
    }

    /// `count` is the width in chunks times the height in chunks, both truncated first.
    /// Truncating the product instead - `((w / chunk) * h) / chunk` - is a different
    /// number whenever the height is not a multiple of the chunk size, and it used to be
    /// what sized the light chunk vector.
    #[test]
    fn test_count_truncates_each_axis_separately() {
        let chunks = Chunks::new((CHUNK_SIZE as u32 * 4, CHUNK_SIZE as u32 * 3 + 5), CHUNK_SIZE);

        assert_eq!(chunks.get_size(), (4, 3));
        assert_eq!(chunks.count(), 12);
        // the last addressable chunk has to be inside the vector `count` sizes
        assert!(chunks.translate_coords(3, 2).unwrap() < chunks.count());
    }

    #[test]
    fn test_chunk_at() {
        let chunks = Chunks::new((CHUNK_SIZE as u32 * 4, CHUNK_SIZE as u32 * 4), CHUNK_SIZE);

        assert_eq!(chunks.chunk_at(0, 0), (0, 0));
        assert_eq!(chunks.chunk_at(CHUNK_SIZE - 1, 0), (0, 0));
        assert_eq!(chunks.chunk_at(CHUNK_SIZE, CHUNK_SIZE * 2), (1, 2));
    }

    /// A chunk size of zero would divide by zero in every method here, so it is raised to
    /// one rather than accepted.
    #[test]
    fn test_chunk_size_is_never_zero() {
        let chunks = Chunks::new((4, 4), 0);

        assert_eq!(chunks.chunk_size(), 1);
        assert_eq!(chunks.count(), 16);
    }

    // --- ChunkTracker ---

    #[test]
    fn test_new_tracker_is_empty() {
        let tracker = ChunkTracker::new(4);
        assert_eq!(tracker.get_num_chunks(), 0);
        tracker.get_oldest_chunk().unwrap_err();
    }

    #[test]
    fn test_out_of_bounds_chunk() {
        let mut tracker = ChunkTracker::new(2);
        tracker.update(2).unwrap_err();
        tracker.remove_chunk(9).unwrap_err();
    }

    #[test]
    fn test_update_then_remove() {
        let mut tracker = ChunkTracker::new(4);

        tracker.update(1).unwrap();
        assert_eq!(tracker.get_num_chunks(), 1);
        assert_eq!(tracker.get_oldest_chunk().unwrap(), 1);

        tracker.remove_chunk(1).unwrap();
        assert_eq!(tracker.get_num_chunks(), 0);
    }

    /// Removing a chunk that was never tracked is a no-op, not an error and not a
    /// corrupted queue.
    #[test]
    fn test_remove_untracked_chunk_is_a_noop() {
        let mut tracker = ChunkTracker::new(4);

        tracker.update(0).unwrap();
        tracker.remove_chunk(2).unwrap();

        assert_eq!(tracker.get_num_chunks(), 1);
        assert_eq!(tracker.get_oldest_chunk().unwrap(), 0);
    }

    /// Touching the same chunk repeatedly must never make it count more than once.
    #[test]
    fn test_repeated_update_counts_once() {
        let mut tracker = ChunkTracker::new(4);

        for _ in 0..5 {
            tracker.update(3).unwrap();
        }

        assert_eq!(tracker.get_num_chunks(), 1);
    }

    /// `0` used to mean "not tracked", colliding with a real elapsed time of 0 - every update
    /// in the tracker's first second. A chunk touched then was never removed from the queue on
    /// its next update, so it appeared twice: `get_num_chunks` over-reported and
    /// `get_oldest_chunk` kept returning a chunk that had just been used. Needs a real second
    /// to pass, since the collision only shows once `as_secs()` moves off 0.
    #[test]
    fn test_chunk_touched_in_the_first_second_is_not_duplicated() {
        let mut tracker = ChunkTracker::new(4);

        // recorded at elapsed time 0
        tracker.update(0).unwrap();
        assert_eq!(tracker.get_num_chunks(), 1);

        std::thread::sleep(std::time::Duration::from_millis(1050));

        // recorded at elapsed time 1, must replace the entry rather than add a second one
        tracker.update(0).unwrap();
        assert_eq!(tracker.get_num_chunks(), 1, "chunk 0 was queued twice under two different times");

        // and the queue must be genuinely empty after removing that one chunk
        tracker.remove_chunk(0).unwrap();
        assert_eq!(tracker.get_num_chunks(), 0, "a stale queue entry survived removal");
    }

    #[test]
    fn test_oldest_chunk_is_the_least_recently_updated() {
        let mut tracker = ChunkTracker::new(4);

        tracker.update(0).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1050));
        tracker.update(1).unwrap();

        assert_eq!(tracker.get_oldest_chunk().unwrap(), 0);

        // Touching chunk 0 again makes chunk 1 the oldest. This needs another second to
        // pass: the tracker's resolution is whole seconds, so an update inside the same
        // second as chunk 1's ties on time and falls back to ordering by chunk index.
        std::thread::sleep(std::time::Duration::from_millis(1050));
        tracker.update(0).unwrap();
        assert_eq!(tracker.get_oldest_chunk().unwrap(), 1);
    }
}
