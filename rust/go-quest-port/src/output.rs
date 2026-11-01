//! Safe rendering of untrusted text in a terminal or a CI log.
//! JSON output deliberately preserves source data; terminal output must not.

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
    use super::escape_terminal;

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
