#![doc = include_str!("../README.md")]
#![no_std]

// Text attributes
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const RESET: &str = "\x1b[0m";

// Foreground colors
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";

// Bold foreground colors
pub const BOLD_RED: &str = "\x1b[1;31m";
pub const BOLD_GREEN: &str = "\x1b[1;32m";

// OSC 8 hyperlinks
/// Start an OSC 8 hyperlink: `\x1b]8;;<url>\x1b\\`
/// Usage: `format!("{LINK_START}{url}{LINK_MID}{text}{LINK_END}")`
pub const LINK_START: &str = "\x1b]8;;";
/// String Terminator that ends the URL parameter of an OSC 8 hyperlink.
pub const LINK_MID: &str = "\x1b\\";
/// Close an OSC 8 hyperlink (empty URI resets).
pub const LINK_END: &str = "\x1b]8;;\x1b\\";
