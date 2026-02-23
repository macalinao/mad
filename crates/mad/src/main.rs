#![doc = include_str!("../README.md")]

use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

use bpaf::Bpaf;

/// mad - Markdown terminal renderer
///
/// Renders Markdown files as beautifully formatted ANSI terminal output with
/// syntax-highlighted code blocks.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version, generate(cli))]
struct Opts {
    #[bpaf(external)]
    command: Command,
}

#[derive(Debug, Clone, Bpaf)]
enum Command {
    #[bpaf(command("man"), hide)]
    /// Generate man page in roff format
    Man,

    Render {
        /// Disable syntax highlighting for code blocks
        #[bpaf(short, long)]
        no_highlight: bool,

        /// Disable background colors on syntax-highlighted code blocks
        #[bpaf(long)]
        no_code_background_color: bool,

        /// Wrap text to specified width (defaults to terminal width)
        #[bpaf(short, long, argument("COLS"))]
        width: Option<usize>,

        /// Markdown file to render (reads from stdin if omitted)
        #[bpaf(positional("FILE"))]
        file: Option<PathBuf>,
    },
}

fn main() {
    let opts = cli().run();

    match opts.command {
        Command::Man => {
            let roff = cli().render_manpage(
                "mad",
                bpaf::doc::Section::General,
                None,
                None,
                Some("Mad Manual"),
            );
            print!("{roff}");
        }
        Command::Render {
            no_highlight,
            no_code_background_color,
            width,
            file,
        } => {
            render(no_highlight, no_code_background_color, width, file);
        }
    }
}

fn render(
    no_highlight: bool,
    no_code_background_color: bool,
    width: Option<usize>,
    file: Option<PathBuf>,
) {
    let input = if let Some(path) = file {
        match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("mad: {}: {e}", path.display());
                std::process::exit(1);
            }
        }
    } else {
        if io::stdin().is_terminal() {
            eprintln!("mad: reading from stdin (use --help for usage)");
        }
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            eprintln!("mad: failed to read stdin: {e}");
            std::process::exit(1);
        }
        buf
    };

    let width = width.or_else(|| {
        terminal_size::terminal_size().map(|(terminal_size::Width(w), _)| usize::from(w))
    });

    let render_opts = markdown_to_ansi::Options {
        syntax_highlight: !no_highlight,
        width,
        code_bg: !no_code_background_color,
    };

    let output = markdown_to_ansi::render(&input, &render_opts);
    print!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parse_no_args() {
        let opts = cli().run_inner(&[]).expect("should parse empty args");
        match opts.command {
            Command::Render {
                no_highlight,
                no_code_background_color,
                width,
                file,
            } => {
                assert!(!no_highlight);
                assert!(!no_code_background_color);
                assert!(width.is_none());
                assert!(file.is_none());
            }
            Command::Man => panic!("expected Render"),
        }
    }

    #[test]
    fn cli_parse_file_arg() {
        let opts = cli()
            .run_inner(&["README.md"])
            .expect("should parse file arg");
        match opts.command {
            Command::Render { file, .. } => {
                assert_eq!(file, Some(PathBuf::from("README.md")));
            }
            Command::Man => panic!("expected Render"),
        }
    }

    #[test]
    fn cli_parse_all_flags() {
        let opts = cli()
            .run_inner(&[
                "--no-highlight",
                "--no-code-background-color",
                "--width",
                "120",
                "test.md",
            ])
            .expect("should parse all flags");
        match opts.command {
            Command::Render {
                no_highlight,
                no_code_background_color,
                width,
                file,
            } => {
                assert!(no_highlight);
                assert!(no_code_background_color);
                assert_eq!(width, Some(120));
                assert_eq!(file, Some(PathBuf::from("test.md")));
            }
            Command::Man => panic!("expected Render"),
        }
    }

    #[test]
    fn cli_parse_short_flags() {
        let opts = cli()
            .run_inner(&["-n", "-w", "80"])
            .expect("should parse short flags");
        match opts.command {
            Command::Render {
                no_highlight,
                no_code_background_color,
                width,
                ..
            } => {
                assert!(no_highlight);
                assert!(!no_code_background_color);
                assert_eq!(width, Some(80));
            }
            Command::Man => panic!("expected Render"),
        }
    }

    #[test]
    fn cli_parse_man_subcommand() {
        let opts = cli()
            .run_inner(&["man"])
            .expect("should parse man subcommand");
        assert!(matches!(opts.command, Command::Man));
    }
}
