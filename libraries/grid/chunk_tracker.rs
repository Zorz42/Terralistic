use anyhow::{anyhow, Result};
use std::collections::BTreeSet;

/// Tracks when each chunk was last touched and hands back the oldest, so a caller can evict
/// what it has not used.
pub struct ChunkTracker {
    /// When each chunk was last touched, or `None` if untracked. An `Option` rather than a `0`
    /// sentinel: `0` is a real elapsed time for the tracker's whole first second.
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
        // Drop the previous entry first, or the chunk is queued twice under two times.
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
