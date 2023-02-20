mod args;

use std::fs;

use clap::Parser;
use phonet::{get_min_filename, Draft};

use crate::args::Args;

fn main() {
    let args = Args::parse();

    // Read file
    let file = fs::read_to_string(&args.file).expect("Could not read phonet file");

    // Parse file
    let draft = Draft::from(&file).expect("Failed to parse file");

    // Minify file
    if args.minify {
        fs::write(get_min_filename(&args.file), &draft.minified)
            .expect("Could not write minified file");
    }

    //TODO Custom tests

    // Run tests and display
    draft.run().display(args.display_level, !args.no_color);

    //TODO Generate words
}
