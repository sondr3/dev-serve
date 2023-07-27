use clap::{Command, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "dev-serve", about, version, author)]
/// Serve a directory with auto-reload
pub struct Cli {
    /// Select port to use
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    /// Auto-reload and watch directory
    #[arg(short, long, default_value_t = true)]
    pub reload: bool,
    /// File extensions to watch
    ///
    /// Must be without the leading `.` and separated by commas, e.g. `html,css,js`
    #[arg(short, long)]
    pub extensions: Option<Vec<String>>,
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long, value_enum)]
    pub completions: Option<Shell>,
    /// Directory to serve
    pub path: Option<PathBuf>,
}

pub fn print_completion<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
