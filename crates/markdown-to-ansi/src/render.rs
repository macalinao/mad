use core::fmt::Write;
use core::mem::take;

use pulldown_cmark::{Alignment, CodeBlockKind, Event, Tag, TagEnd};

use crate::wrap::wrap_text;
use ansi_term_styles::{BLUE, BOLD, DIM, ITALIC, LINK_END, LINK_MID, LINK_START, RESET, UNDERLINE};

/// State tracked while walking pulldown-cmark events.
pub(crate) struct RenderState {
    text_buf: String,
    code_buf: String,
    code_lang: String,
    in_code_block: bool,
    in_link: bool,
    link_url: String,
    link_text: String,
    /// `None` = unordered, `Some(start)` = ordered.
    list_stack: Vec<Option<u64>>,
    list_counters: Vec<u64>,
    in_item: bool,
    style_stack: Vec<&'static str>,
    /// Columns consumed by the list item prefix (e.g. 2 for `- `).
    /// Reset to 0 on the next `flush_text`.
    line_prefix_width: usize,
    /// Continuation indent for wrapped lines inside a list item.
    item_cont_indent: usize,
    // Table state
    table: Option<TableState>,
}

/// Accumulated table data collected during parsing.
struct TableState {
    alignments: Vec<Alignment>,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    in_head: bool,
}

impl RenderState {
    fn new() -> Self {
        Self {
            text_buf: String::new(),
            code_buf: String::new(),
            code_lang: String::new(),
            in_code_block: false,
            in_link: false,
            link_url: String::new(),
            link_text: String::new(),
            list_stack: Vec::new(),
            list_counters: Vec::new(),
            in_item: false,
            style_stack: Vec::new(),
            line_prefix_width: 0,
            item_cont_indent: 0,
            table: None,
        }
    }
}

#[allow(clippy::too_many_lines)] // Event-loop match is inherently long; splitting would hurt readability.
pub(crate) fn render_events<'a>(
    parser: impl Iterator<Item = Event<'a>>,
    opts: &crate::Options,
    inline_mode: bool,
) -> String {
    let mut out = String::new();
    let mut st = RenderState::new();

    for event in parser {
        match event {
            // --- Block-level containers ---
            Event::End(TagEnd::Paragraph) => {
                if !inline_mode {
                    flush_text(&mut out, &mut st, opts.width);
                    if st.list_stack.is_empty() {
                        out.push_str("\n\n");
                    } else {
                        out.push('\n');
                    }
                }
            }

            Event::Start(Tag::Heading { .. }) => {
                flush_text(&mut out, &mut st, opts.width);
                st.text_buf.push_str(BOLD);
            }
            Event::End(TagEnd::Heading(_)) => {
                st.text_buf.push_str(RESET);
                flush_text(&mut out, &mut st, opts.width);
                out.push('\n');
            }

            Event::Start(Tag::CodeBlock(kind)) => {
                st.in_code_block = true;
                st.code_buf.clear();
                st.code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        lang.split_whitespace().next().unwrap_or("").to_string()
                    }
                    CodeBlockKind::Indented => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                flush_code_block(&mut out, &mut st, opts);
            }

            Event::Start(Tag::List(start)) => {
                st.list_stack.push(start);
                st.list_counters.push(start.unwrap_or(1));
            }
            Event::End(TagEnd::List(_)) => {
                st.list_stack.pop();
                st.list_counters.pop();
                if st.list_stack.is_empty() {
                    out.push('\n');
                }
            }

            Event::Start(Tag::Item) => {
                begin_list_item(&mut out, &mut st, opts.width);
            }
            Event::End(TagEnd::Item) => {
                flush_text(&mut out, &mut st, opts.width);
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                st.in_item = false;
                st.item_cont_indent = 0;
            }

            // --- Tables ---
            Event::Start(Tag::Table(alignments)) => {
                flush_text(&mut out, &mut st, opts.width);
                st.table = Some(TableState {
                    alignments: alignments.clone(),
                    header: Vec::new(),
                    rows: Vec::new(),
                    current_row: Vec::new(),
                    in_head: false,
                });
            }
            Event::End(TagEnd::Table) => {
                if let Some(table) = st.table.take() {
                    render_table(&mut out, &table);
                    out.push('\n');
                }
            }
            Event::Start(Tag::TableHead) => {
                if let Some(ref mut table) = st.table {
                    table.in_head = true;
                    table.current_row.clear();
                }
            }
            Event::End(TagEnd::TableHead) => {
                if let Some(ref mut table) = st.table {
                    table.header = take(&mut table.current_row);
                    table.in_head = false;
                }
            }
            Event::Start(Tag::TableRow) => {
                if let Some(ref mut table) = st.table {
                    table.current_row.clear();
                }
            }
            Event::End(TagEnd::TableRow) => {
                if let Some(ref mut table) = st.table {
                    let row = take(&mut table.current_row);
                    table.rows.push(row);
                }
            }
            Event::Start(Tag::TableCell) => {
                st.text_buf.clear();
            }
            Event::End(TagEnd::TableCell) => {
                if let Some(ref mut table) = st.table {
                    table.current_row.push(take(&mut st.text_buf));
                }
            }

            // --- Inline elements ---
            Event::Text(t) => {
                if st.in_code_block {
                    st.code_buf.push_str(&t);
                } else if st.in_link {
                    st.link_text.push_str(&t);
                } else {
                    st.text_buf.push_str(&t);
                }
            }

            Event::Code(t) => {
                if st.in_link {
                    st.link_text.push_str(&t);
                } else {
                    let _ = write!(st.text_buf, "{BLUE}{t}{RESET}");
                }
            }

            Event::Start(Tag::Strong) => {
                st.text_buf.push_str(BOLD);
                st.style_stack.push(BOLD);
            }
            Event::End(TagEnd::Strong | TagEnd::Emphasis) => {
                st.text_buf.push_str(RESET);
                st.style_stack.pop();
            }

            Event::Start(Tag::Emphasis) => {
                st.text_buf.push_str(ITALIC);
                st.style_stack.push(ITALIC);
            }

            Event::Start(Tag::Link { dest_url, .. }) => {
                st.in_link = true;
                st.link_url = dest_url.to_string();
                st.link_text.clear();
            }
            Event::End(TagEnd::Link) => {
                let text = take(&mut st.link_text);
                let url = take(&mut st.link_url);
                st.in_link = false;
                let _ = write!(
                    st.text_buf,
                    "{LINK_START}{url}{LINK_MID}{UNDERLINE}{text}{RESET}{LINK_END}"
                );
            }

            Event::SoftBreak => {
                if st.in_code_block {
                    st.code_buf.push('\n');
                } else {
                    st.text_buf.push(' ');
                }
            }
            Event::HardBreak => {
                if st.in_code_block {
                    st.code_buf.push('\n');
                } else {
                    st.text_buf.push('\n');
                }
            }

            _ => {}
        }
    }

    flush_text(&mut out, &mut st, opts.width);
    trim_trailing_blank_lines(out)
}

/// Emit the list item prefix and prepare indentation state.
fn begin_list_item(out: &mut String, st: &mut RenderState, width: Option<usize>) {
    flush_text(out, st, width);
    let depth = st.list_stack.len().saturating_sub(1);
    let indent = "  ".repeat(depth);
    let prefix = if let (Some(Some(_)), Some(counter)) =
        (st.list_stack.last(), st.list_counters.last_mut())
    {
        let p = format!("{indent}{counter}. ");
        *counter += 1;
        p
    } else {
        format!("{indent}- ")
    };
    st.line_prefix_width = prefix.len();
    st.item_cont_indent = prefix.len();
    out.push_str(&prefix);
    st.in_item = true;
}

/// Collapse trailing blank lines: keep at most one trailing newline.
fn trim_trailing_blank_lines(mut out: String) -> String {
    let trimmed = out.trim_end_matches('\n');
    if trimmed.is_empty() {
        return String::new();
    }
    let had_newline = out.ends_with('\n');
    out.truncate(trimmed.len());
    if had_newline {
        out.push('\n');
    }
    out
}

/// Flush a completed fenced code block to the output buffer.
fn flush_code_block(out: &mut String, st: &mut RenderState, opts: &crate::Options) {
    let code = take(&mut st.code_buf);
    let lang = take(&mut st.code_lang);
    st.in_code_block = false;

    if opts.syntax_highlight {
        out.push_str(&crate::highlight::highlight_code_block(
            &code,
            &lang,
            opts.width,
            opts.code_bg,
        ));
    } else if lang.is_empty() {
        out.push_str("```\n");
        out.push_str(&code);
        out.push_str("```\n");
    } else {
        let _ = writeln!(out, "```{lang}");
        out.push_str(&code);
        out.push_str("```\n");
    }
    out.push('\n');
}

/// Flush accumulated inline text to the output buffer, optionally word-wrapping.
fn flush_text(out: &mut String, st: &mut RenderState, width: Option<usize>) {
    if !st.text_buf.is_empty() {
        if let Some(w) = width {
            let first_offset = take(&mut st.line_prefix_width);
            let cont_indent = st.item_cont_indent;
            out.push_str(&wrap_text(&st.text_buf, w, first_offset, cont_indent));
        } else {
            out.push_str(&st.text_buf);
        }
        st.text_buf.clear();
    }
}

/// Compute the visible width of a string, stripping ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    crate::highlight::visible_width(s)
}

/// Render a parsed table with Unicode box-drawing borders.
fn render_table(out: &mut String, table: &TableState) {
    let num_cols = table.alignments.len();
    if num_cols == 0 {
        return;
    }

    // Calculate column widths from header and all rows.
    let mut col_widths = vec![0usize; num_cols];
    for (i, cell) in table.header.iter().enumerate() {
        col_widths[i] = col_widths[i].max(visible_width(cell));
    }
    for row in &table.rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                col_widths[i] = col_widths[i].max(visible_width(cell));
            }
        }
    }

    // Draw top border: ┌───┬───┐
    draw_border(
        out,
        &col_widths,
        &BorderChars {
            left: '┌',
            mid: '┬',
            right: '┐',
        },
    );

    // Draw header row with bold text
    draw_row(
        out,
        &RowCtx {
            cells: &table.header,
            widths: &col_widths,
            alignments: &table.alignments,
            is_header: true,
        },
    );

    // Draw header separator: ├───┼───┤
    draw_border(
        out,
        &col_widths,
        &BorderChars {
            left: '├',
            mid: '┼',
            right: '┤',
        },
    );

    // Draw body rows
    for row in &table.rows {
        draw_row(
            out,
            &RowCtx {
                cells: row,
                widths: &col_widths,
                alignments: &table.alignments,
                is_header: false,
            },
        );
    }

    // Draw bottom border: └───┴───┘
    draw_border(
        out,
        &col_widths,
        &BorderChars {
            left: '└',
            mid: '┴',
            right: '┘',
        },
    );
}

/// Box-drawing characters for a horizontal table border.
struct BorderChars {
    left: char,
    mid: char,
    right: char,
}

/// Draw a horizontal border line using box-drawing characters.
fn draw_border(out: &mut String, widths: &[usize], chars: &BorderChars) {
    let _ = write!(out, "{DIM}");
    out.push(chars.left);
    for (i, &w) in widths.iter().enumerate() {
        for _ in 0..w + 2 {
            out.push('─');
        }
        if i < widths.len() - 1 {
            out.push(chars.mid);
        }
    }
    out.push(chars.right);
    let _ = write!(out, "{RESET}");
    out.push('\n');
}

/// Context for rendering a single table row.
struct RowCtx<'a> {
    cells: &'a [String],
    widths: &'a [usize],
    alignments: &'a [Alignment],
    is_header: bool,
}

/// Draw a single table row with padding and alignment.
fn draw_row(out: &mut String, ctx: &RowCtx<'_>) {
    for (i, width) in ctx.widths.iter().enumerate() {
        let _ = write!(out, "{DIM}│{RESET} ");
        let cell = ctx.cells.get(i).map_or("", String::as_str);
        let vis_w = visible_width(cell);
        let padding = width.saturating_sub(vis_w);
        let align = ctx.alignments.get(i).copied().unwrap_or(Alignment::None);

        let (left_pad, right_pad) = match align {
            Alignment::Center => (padding / 2, padding - padding / 2),
            Alignment::Right => (padding, 0),
            Alignment::Left | Alignment::None => (0, padding),
        };

        for _ in 0..left_pad {
            out.push(' ');
        }
        if ctx.is_header {
            let _ = write!(out, "{BOLD}{cell}{RESET}");
        } else {
            out.push_str(cell);
        }
        for _ in 0..right_pad {
            out.push(' ');
        }
        out.push(' ');
    }
    let _ = write!(out, "{DIM}│{RESET}");
    out.push('\n');
}
