use crate::cli::{print_completion, Cli};

mod cli;

use anyhow::Result;
use clap::{CommandFactory, Parser};

fn main() -> Result<()> {
    let opts = Cli::parse();

    if let Some(comp) = opts.completions {
        let mut app = Cli::command();
        print_completion(comp, &mut app);
        return Ok(());
    }

    Ok(())
}
