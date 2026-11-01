//! Safe rendering of untrusted text in a terminal or a CI log.
//! JSON output deliberately preserves source data; terminal output must not.

use std::io::{self, Write};

/// Maximum size of a single buffered machine-readable JSON output report.
pub const MAX_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

/// Abort oversized JSON output before writing any part of it to stdout.
/// This preserves valid one-document stdout for downstream command pipelines.
pub struct LimitedWriter {
    bytes: Vec<u8>,
    limit: usize,
}

impl LimitedWriter {
    pub fn new(limit: usize) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for LimitedWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if data.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("JSON report exceeds output size limit"));
        }
        self.bytes.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Visible, printable replacement for formatting and terminal-control characters.
pub fn escape_terminal(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        let cp = ch as u32;
        let dangerous = ch.is_control()
            || (0x200B..=0x200F).contains(&cp)
            || (0x202A..=0x202E).contains(&cp)
            || (0x2060..=0x2069).contains(&cp)
            || cp == 0x061C
            || cp == 0xFEFF;
        if dangerous {
            use std::fmt::Write;
            let _ = write!(out, "\\u{{{cp:04X}}}");
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{escape_terminal, LimitedWriter};
    use std::io::Write;

    #[test]
    fn limited_writer_does_not_emit_partial_oversized_report() {
        let mut writer = LimitedWriter::new(4);
        writer.write_all(b"ABCD").unwrap();
        assert!(writer.write_all(b"E").is_err());
        assert_eq!(writer.into_bytes(), b"ABCD");
    }

    #[test]
    fn zero_limit_rejects_nonempty_output() {
        let mut writer = LimitedWriter::new(0);
        assert!(writer.write_all(b"x").is_err());
        assert_eq!(writer.into_bytes().len(), 0);
    }

    #[test]
    fn escapes_ansi_osc_cr_lf_and_bidi_controls() {
        let raw = "Game\x1b[2J\x1b]0;fake\x07\r\n\u{202E}\u{2066}OK";
        let visible = escape_terminal(raw);
        assert!(!visible.contains('\x1b'));
        assert!(!visible.contains('\n'));
        assert!(!visible.contains('\r'));
        assert!(!visible.contains('\u{202E}'));
        assert!(visible.contains("\\u{001B}"));
        assert!(visible.contains("\\u{202E}"));
        assert!(visible.contains("OK"));
    }

    #[test]
    fn preserves_normal_unicode_game_titles() {
        let input = "Pokémon • مرحباً • 日本語";
        assert_eq!(escape_terminal(input), input);
    }

    #[test]
    fn keeps_carriage_return_out_of_log_lines() {
        assert_eq!(
            escape_terminal("title\rstatus: success"),
            "title\\u{000D}status: success"
        );
    }
}
