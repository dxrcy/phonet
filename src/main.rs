mod args;

use std::fs;

use args::Args;
use clap::Parser;

use phonet::Draft;

fn main() {
    let args = Args::parse();

    // Read file
    let file = fs::read_to_string(&args.file).expect("Could not read phonet file");

    // Parse file
    let draft = Draft::from(&file).expect("Failed to parse file");

    //TODO Minify

    //TODO Custom tests

    // Run tests and display
    draft.run().display(args.display_level, !args.no_color);

    //TODO Generate words
}
