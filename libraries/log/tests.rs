#![allow(clippy::unwrap_used, clippy::indexing_slicing)] // the wrong number of lines is a test failure
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::libraries::log::{clear_sink, format_line, format_timestamp, log, set_sink, LogLevel};

    /// The sink is process-global, so the tests that touch it have to take turns - libtest
    /// runs them on parallel threads within one process.
    static SINK_LOCK: Mutex<()> = Mutex::new(());

    fn capture(body: impl FnOnce()) -> Vec<(LogLevel, String)> {
        let _guard = SINK_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        clear_sink();

        let lines = Arc::new(Mutex::new(Vec::new()));
        let sink = lines.clone();
        set_sink(Box::new(move |level, line| {
            sink.lock().unwrap().push((level, line.to_owned()));
        }));

        body();

        clear_sink();
        let captured = lines.lock().unwrap().clone();
        captured
    }

    #[test]
    fn test_a_timestamp_is_prefixed() {
        let line = format_timestamp("hello");

        assert!(line.ends_with("hello"));
        assert!(line.starts_with('['), "the timestamp goes first: {line}");
        // [MM-DD HH:MM:SS] is 15 characters plus the brackets
        assert!(line.len() > "hello".len() + 15, "no timestamp in {line}");
    }

    #[test]
    fn test_a_line_carries_its_level() {
        assert!(format_line(LogLevel::Info, "x").contains("[INFO]"));
        assert!(format_line(LogLevel::Warning, "x").contains("[WARNING]"));
        assert!(format_line(LogLevel::Error, "x").contains("[ERROR]"));
    }

    #[test]
    fn test_the_sink_receives_the_formatted_line() {
        let lines = capture(|| log(LogLevel::Warning, "careful"));

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].0, LogLevel::Warning);
        assert!(lines[0].1.contains("[WARNING]"));
        assert!(lines[0].1.ends_with("careful"));
    }

    /// A multi-line message becomes one timestamped line each, rather than one line with the
    /// rest hanging off the end of it.
    #[test]
    fn test_a_multiline_message_becomes_one_line_each() {
        let lines = capture(|| log(LogLevel::Info, "first\nsecond\nthird"));

        assert_eq!(lines.len(), 3);
        assert!(lines[0].1.ends_with("first"));
        assert!(lines[2].1.ends_with("third"));
        for (_level, line) in &lines {
            assert!(line.contains("[INFO]"), "every line gets its own tag: {line}");
        }
    }

    #[test]
    fn test_an_empty_message_logs_nothing() {
        assert!(capture(|| log(LogLevel::Info, "")).is_empty());
    }

    /// The first sink installed wins, so a second server in one process cannot steal the
    /// first one's output.
    #[test]
    fn test_a_second_sink_is_ignored() {
        let _guard = SINK_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        clear_sink();

        let first = Arc::new(Mutex::new(Vec::new()));
        let second = Arc::new(Mutex::new(Vec::new()));

        let sink = first.clone();
        set_sink(Box::new(move |_level, line| sink.lock().unwrap().push(line.to_owned())));
        let sink = second.clone();
        set_sink(Box::new(move |_level, line| sink.lock().unwrap().push(line.to_owned())));

        log(LogLevel::Info, "who gets this");

        assert_eq!(first.lock().unwrap().len(), 1);
        assert!(second.lock().unwrap().is_empty(), "the second sink should have been ignored");
        clear_sink();
    }
}
