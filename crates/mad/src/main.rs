#![doc = include_str!("../README.md")]

use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};

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

        /// Do not pipe output through a pager
        #[bpaf(long)]
        no_pager: bool,

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
            no_pager,
            width,
            file,
        } => {
            let input = read_input(file);

            let term_size = terminal_size::terminal_size();
            let width =
                width.or_else(|| term_size.map(|(terminal_size::Width(w), _)| usize::from(w)));
            let term_height = term_size.map(|(_, terminal_size::Height(h))| usize::from(h));

            let render_opts = markdown_to_ansi::Options {
                syntax_highlight: !no_highlight,
                width,
                code_bg: !no_code_background_color,
            };

            let output = markdown_to_ansi::render(&input, &render_opts);
            print_or_page(&output, no_pager, term_height);
        }
    }
}

fn read_input(file: Option<PathBuf>) -> String {
    if let Some(path) = file {
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
    }
}

fn print_or_page(output: &str, no_pager: bool, term_height: Option<usize>) {
    if no_pager || !io::stdout().is_terminal() {
        print!("{output}");
        return;
    }

    if let Some(height) = term_height {
        let line_count = output.lines().count();
        if line_count <= height {
            print!("{output}");
            return;
        }
    }

    if spawn_pager(output).is_err() {
        print!("{output}");
    }
}

fn spawn_pager(output: &str) -> io::Result<()> {
    let pager_env = std::env::var("PAGER").unwrap_or_else(|_| "less".to_string());
    let mut parts = pager_env.split_whitespace();
    let cmd = parts.next().unwrap_or("less");
    let mut args: Vec<&str> = parts.collect();

    // Ensure less gets -R for ANSI escape code passthrough
    if cmd.ends_with("less") && !args.iter().any(|a| a.contains('R')) {
        args.push("-R");
    }

    let mut child = ProcessCommand::new(cmd)
        .args(&args)
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        // Ignore broken pipe — user quit the pager early
        let _ = stdin.write_all(output.as_bytes());
    }

    let _ = child.wait();
    Ok(())
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
                no_pager,
                width,
                file,
            } => {
                assert!(!no_highlight);
                assert!(!no_code_background_color);
                assert!(!no_pager);
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
                "--no-pager",
                "--width",
                "120",
                "test.md",
            ])
            .expect("should parse all flags");
        match opts.command {
            Command::Render {
                no_highlight,
                no_code_background_color,
                no_pager,
                width,
                file,
            } => {
                assert!(no_highlight);
                assert!(no_code_background_color);
                assert!(no_pager);
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
