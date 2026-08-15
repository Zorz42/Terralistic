use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// How long a test waits before giving up. Generous, because it only matters when something
/// is broken - a passing test reaches its condition in milliseconds.
pub const TIMEOUT: Duration = Duration::from_secs(30);

/// A port no other test in this process is using, and that nothing else holds right now.
///
/// **Asking the OS for port 0 is not enough**: two tests probing one after the other can be
/// handed the same port, and then one test's client joins the other test's server. The
/// counter makes the numbers distinct and the bind only skips ports held from outside. The
/// range sits below the ephemeral one and clear of the ports the game uses.
pub fn free_port() -> u16 {
    static NEXT_PORT: AtomicU64 = AtomicU64::new(0);
    const FIRST_PORT: u64 = 41_000;
    const PORT_COUNT: u64 = 4_000;

    for _ in 0..PORT_COUNT {
        let port = (FIRST_PORT + NEXT_PORT.fetch_add(1, Ordering::Relaxed) % PORT_COUNT) as u16;
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            drop(listener);
            return port;
        }
    }
    panic!("no free port in the test range");
}

/// Runs `step` until it returns true, or panics with `what` at the timeout.
///
/// Panicking rather than hanging is the point: with no per-test timeout, a wait that never ends is
/// the difference between a red build and a job that runs until CI kills it.
#[track_caller]
pub fn wait_until(what: &str, mut step: impl FnMut() -> bool) {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if step() {
            return;
        }
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// A directory under the system temp dir that deletes itself when it goes out of scope. All
/// the tests need of `tempfile`, so it is spelled out rather than pulled in.
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    #[must_use]
    pub fn new(tag: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let path = std::env::temp_dir().join(format!("terralistic-test-{tag}-{}-{}", std::process::id(), COUNTER.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir_all(&path).expect("could not create a temp dir for the test");
        Self { path }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // Best effort: a failed cleanup must not turn a passing test red.
        drop(std::fs::remove_dir_all(&self.path));
    }
}
