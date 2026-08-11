#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::client::game::chunk_tracker::ChunkTracker;

    #[test]
    fn test_new_tracker_is_empty() {
        let tracker = ChunkTracker::new(4);
        assert_eq!(tracker.get_num_chunks(), 0);
        assert!(tracker.get_oldest_chunk().is_err());
    }

    #[test]
    fn test_out_of_bounds_chunk() {
        let mut tracker = ChunkTracker::new(2);
        assert!(tracker.update(2).is_err());
        assert!(tracker.remove_chunk(9).is_err());
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

    /// The tracker used to store `0` to mean "not tracked", which collides with a real
    /// elapsed time of 0 seconds - that is every update during the tracker's first
    /// second of life. A chunk first touched in that window was never removed from the
    /// queue on its next update, so it appeared twice under two different times:
    /// `get_num_chunks` over-reported and `get_oldest_chunk` kept returning a chunk that
    /// had just been used.
    ///
    /// This needs a real second to pass, since the collision only shows up once
    /// `timer.elapsed().as_secs()` moves off 0.
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
