use std::sync::LazyLock;

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use ansi_term_styles::RESET;

static DEFAULTS: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

/// Extra syntax set for languages not in syntect's defaults (e.g. TOML).
/// Precompiled at build time via the `sublime-syntaxes` crate.
static EXTRAS: LazyLock<SyntaxSet> = LazyLock::new(sublime_syntaxes::extra_syntax_set);

pub(crate) static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Look up a syntax by language token, searching defaults first, then extras.
fn find_syntax(
    lang: &str,
) -> (
    &'static SyntaxSet,
    &'static syntect::parsing::SyntaxReference,
) {
    if lang.is_empty() {
        return (&*DEFAULTS, DEFAULTS.find_syntax_plain_text());
    }
    if let Some(s) = DEFAULTS.find_syntax_by_token(lang) {
        return (&*DEFAULTS, s);
    }
    if let Some(s) = EXTRAS.find_syntax_by_token(lang) {
        return (&*EXTRAS, s);
    }
    (&*DEFAULTS, DEFAULTS.find_syntax_plain_text())
}

/// Returns true if a syntax definition exists for the given language token.
pub fn has_syntax(lang: &str) -> bool {
    DEFAULTS.find_syntax_by_token(lang).is_some() || EXTRAS.find_syntax_by_token(lang).is_some()
}

/// Compute the visible column width of a string, handling tabs (4-column stops)
/// and skipping ANSI CSI (`\x1b[...letter`) and OSC (`\x1b]...ST`) escape sequences.
pub(crate) fn visible_width(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut width = 0;
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        if ch == '\x1b' && i + 1 < len {
            let next = chars[i + 1];
            if next == '[' {
                // CSI: skip until ASCII letter
                i += 2;
                while i < len {
                    let done = chars[i].is_ascii_alphabetic();
                    i += 1;
                    if done {
                        break;
                    }
                }
            } else if next == ']' {
                // OSC: skip until ST (\x1b\\) or BEL (\x07)
                i += 2;
                while i < len {
                    if chars[i] == '\x07' {
                        i += 1;
                        break;
                    }
                    if chars[i] == '\x1b' && i + 1 < len && chars[i + 1] == '\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            } else {
                // Unknown escape, skip the \x1b and next char
                i += 2;
            }
            continue;
        }

        if ch == '\t' {
            width = (width + 4) & !3;
        } else if ch != '\n' && ch != '\r' {
            width += 1;
        }
        i += 1;
    }
    width
}

/// Syntax-highlight a code block using syntect, with background padding.
///
/// When color or syntax highlighting is disabled, returns the code unchanged.
/// Falls back to plain text if the language is unknown.
///
/// The background extends from the first content column to `width` columns.
/// The caller is responsible for prepending any indent before each output line.
pub(crate) fn highlight_code_block(
    code: &str,
    lang: &str,
    width: Option<usize>,
    code_bg: bool,
) -> String {
    let (syntax_set, syntax) = find_syntax(lang);
    let theme = &THEME_SET.themes["base16-ocean.dark"];
    let mut h = syntect::easy::HighlightLines::new(syntax, theme);
    let mut out = String::new();

    let full_width = width.unwrap_or(80);

    let bg = if code_bg {
        theme
            .settings
            .background
            .map(|c| format!("\x1b[48;2;{};{};{}m", c.r, c.g, c.b))
    } else {
        None
    };

    for line in syntect::util::LinesWithEndings::from(code) {
        match h.highlight_line(line, syntax_set) {
            Ok(ranges) => {
                let highlighted = syntect::util::as_24_bit_terminal_escaped(&ranges, code_bg);
                let highlighted = highlighted.trim_end_matches('\n');

                if let Some(ref bg_code) = bg {
                    let content_width = visible_width(highlighted);
                    let padding = full_width.saturating_sub(content_width);
                    out.push_str(bg_code);
                    out.push_str(highlighted);
                    out.extend(core::iter::repeat_n(' ', padding));
                } else {
                    out.push_str(highlighted);
                }
                out.push_str(RESET);
                out.push('\n');
            }
            Err(_) => out.push_str(line),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_ascii() {
        assert_eq!(visible_width("hello"), 5);
    }

    #[test]
    fn visible_width_tabs() {
        // Tab at column 0 advances to column 4
        assert_eq!(visible_width("\t"), 4);
        // "ab" is 2 columns, tab advances to column 4
        assert_eq!(visible_width("ab\t"), 4);
        // "abcd" is 4 columns, tab advances to column 8
        assert_eq!(visible_width("abcd\t"), 8);
    }

    #[test]
    fn visible_width_strips_ansi() {
        assert_eq!(visible_width("\x1b[31mhello\x1b[0m"), 5);
        assert_eq!(visible_width("\x1b[38;2;100;200;50mfoo\x1b[0m"), 3);
    }

    #[test]
    fn visible_width_empty() {
        assert_eq!(visible_width(""), 0);
        assert_eq!(visible_width("\n"), 0);
    }

    #[test]
    fn visible_width_osc8_hyperlink() {
        // OSC 8 hyperlink: \x1b]8;;url\x1b\\visible text\x1b]8;;\x1b\\
        let link = "\x1b]8;;https://example.com\x1b\\docs\x1b]8;;\x1b\\";
        assert_eq!(visible_width(link), 4, "Only 'docs' should count");
    }

    #[test]
    fn visible_width_osc8_with_ansi_styling() {
        // Styled hyperlink: CSI + OSC 8 combined
        let link = "\x1b]8;;https://example.com\x1b\\\x1b[4mdocs\x1b[0m\x1b]8;;\x1b\\";
        assert_eq!(visible_width(link), 4, "Only 'docs' should count");
    }
}
