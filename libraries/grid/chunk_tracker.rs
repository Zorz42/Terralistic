use anyhow::{anyhow, Result};
use std::collections::BTreeSet;

/// Tracks chunks and their modification time
/// allows you to get the earliest modified chunk
/// used to delete unused chunks to save memory
pub struct ChunkTracker {
    /// Time each chunk was last touched, or `None` if it is not currently tracked.
    /// This has to be an `Option` rather than a `0` sentinel, because `0` is a
    /// legitimate elapsed time for the whole first second the tracker is alive.
    modified_time: Vec<Option<u32>>,
    timer: std::time::Instant,
    queue: BTreeSet<(u32, usize)>,
}

impl ChunkTracker {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            modified_time: vec![None; size],
            timer: std::time::Instant::now(),
            queue: BTreeSet::new(),
        }
    }

    fn get_modified_time(&mut self, chunk: usize) -> Result<&mut Option<u32>> {
        self.modified_time.get_mut(chunk).ok_or_else(|| anyhow!("Chunk out of bounds"))
    }

    pub fn update(&mut self, chunk: usize) -> Result<()> {
        let time = self.timer.elapsed().as_secs() as u32;
        // drop the previous queue entry first, otherwise the chunk would be left in the
        // queue twice under two different times
        self.remove_chunk(chunk)?;
        *self.get_modified_time(chunk)? = Some(time);
        self.queue.insert((time, chunk));
        Ok(())
    }

    pub fn get_oldest_chunk(&self) -> Result<usize> {
        self.queue.first().ok_or_else(|| anyhow!("No chunks in queue")).map(|&(_, chunk)| chunk)
    }

    #[must_use]
    pub fn get_num_chunks(&self) -> usize {
        self.queue.len()
    }

    pub fn remove_chunk(&mut self, chunk: usize) -> Result<()> {
        if let Some(time) = *self.get_modified_time(chunk)? {
            self.queue.remove(&(time, chunk));
            *self.get_modified_time(chunk)? = None;
        }
        Ok(())
    }
}
