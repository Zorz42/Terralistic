use std::sync::{Mutex, PoisonError};

/// How bad the thing being reported is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl LogLevel {
    /// The tag a line is prefixed with.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Info => "[INFO]",
            Self::Warning => "[WARNING]",
            Self::Error => "[ERROR]",
        }
    }
}

/// Prefixes a message with the current local time, as `[MM-DD HH:MM:SS]`.
///
/// A clock that cannot be read gives `???` rather than failing: a log line is not worth
/// propagating an error for, and a line with no time on it is still the line.
#[must_use]
pub fn format_timestamp(message: &str) -> String {
    let timestamp = chrono::Local::now().naive_local().and_utc().timestamp();
    let timestamp = chrono::DateTime::from_timestamp(timestamp, 0);
    format!("[{}] {message}", timestamp.map_or_else(|| "???".to_owned(), |time| time.format("%m-%d %H:%M:%S").to_string()))
}

/// A whole line: level tag, timestamp, message.
#[must_use]
pub fn format_line(level: LogLevel, message: &str) -> String {
    format_timestamp(&format!("{} {message}", level.tag()))
}

/// Somewhere for log lines to go, besides the terminal.
type Sink = Box<dyn Fn(LogLevel, &str) + Send>;

/// The process-wide extra destination for log lines.
///
/// **This is global because the code that logs usually has nothing to reach through.** A
/// free function called from all over a server has no server handle, and threading one to
/// every call site to carry a `Sender` is a worse trade than this. The first sink installed
/// wins and later ones are ignored, so a second server in the same process does not steal
/// the first one's output.
///
/// A `Box<dyn Fn>` is `Send` but not `Sync`, so this needs the `Mutex` rather than a
/// `OnceLock`.
static SINK: Mutex<Option<Sink>> = Mutex::new(None);

/// Installs the extra destination, if there is not one already.
pub fn set_sink(sink: Sink) {
    let mut slot = SINK.lock().unwrap_or_else(PoisonError::into_inner);
    if slot.is_none() {
        *slot = Some(sink);
    }
}

/// Forgets the installed sink. Only for tests, which would otherwise have the first test to
/// run decide where every later test's output goes.
#[cfg(test)]
pub fn clear_sink() {
    *SINK.lock().unwrap_or_else(PoisonError::into_inner) = None;
}

/// Prints one line to the terminal and hands it to the sink.
///
/// **A message containing newlines becomes one line each**, so a multi-line report is still
/// a sequence of timestamped lines rather than one line with the rest hanging off the end of
/// it. An empty message is nothing at all.
pub fn log(level: LogLevel, message: &str) {
    if message.is_empty() {
        return;
    }

    if message.contains('\n') {
        for line in message.split('\n') {
            log(level, line);
        }
        return;
    }

    let line = format_line(level, message);
    println!("{line}");

    if let Some(sink) = SINK.lock().unwrap_or_else(PoisonError::into_inner).as_ref() {
        sink(level, &line);
    }
}
