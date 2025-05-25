use std::fs;

use clap::Parser;
use color_eyre::Result;

mod aheui;
mod hangul;

use aheui::Aheui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let contents = fs::read_to_string(args.file).expect("Failed to read file.");

    let mut parser = Aheui::new(contents);

    parser.debug = args.debug;

    // println!("{:?}", parser.src_mat);

    loop {
        parser.next();

        if args.debug {
            parser.print_state();
        }

        if parser.ended {
            break;
        }
    }

    Ok(())
}

/// Debuggable Aheui Interpreter
#[derive(Parser)]
struct Args {
    /// Print debug
    #[arg(long, short)]
    debug: bool,

    /// Aheui source code
    file: std::path::PathBuf,
}
