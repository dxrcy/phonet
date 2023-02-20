mod args;

use std::fs;

use clap::Parser;
use phonet::{get_min_filename, Draft, Message::Test, TestDraft};

use crate::args::Args;

fn main() {
    let args = Args::parse();

    // Read file
    let file = fs::read_to_string(&args.file).expect("Could not read phonet file");

    // Parse file
    let mut draft = Draft::from(&file).expect("Failed to parse file");

    // Use custom CLI tests if given
    if let Some(tests) = args.tests {
        draft.messages = tests
            .split(',')
            .map(|x| {
                Test(TestDraft {
                    intent: true,
                    word: x.to_string(),
                })
            })
            .collect();
    }

    // Minify file
    if args.minify {
        fs::write(
            get_min_filename(&args.file),
            &draft.minify(args.with_tests).expect("Failed to minify"),
        )
        .expect("Could not write minified file");
    }

    // Run tests and display
    draft.run().display(args.display_level, !args.no_color);

    //TODO Generate words
}
