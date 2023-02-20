mod args;

use std::fs;

use clap::Parser;
use colorful::Color;

use phonet::{color, get_min_filename, Draft, Message::Test, TestDraft};

use crate::args::Args;

/// Unwrap the `Ok` value of a `Result`, or exit with a stringified `Error`
///
/// TODO something else????
macro_rules! try_this {
    ( $result: expr ) => {{
        match $result {
            Ok(value) => value,

            Err(err) => {
                return Err(err.to_string());
            }
        }
    }};
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    // Read file
    let file = fs::read_to_string(&args.file).expect("Could not read phonet file");

    // Parse file
    let mut draft = try_this!(Draft::from(&file));

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

    // Generate words
    let generated = if let Some(count) = args.generate {
        // Default count to 1 word
        let count = count.unwrap_or(1);

        // Min and max length
        let length = args.generate_min_len.unwrap_or(3)..args.generate_max_len.unwrap_or(14);

        // Generate words
        Some(try_this!(draft.generate(count, length)))
    } else {
        None
    };

    // Run tests and display
    draft.run().display(args.display_level, !args.no_color);

    // Display generated words
    if let Some(words) = generated {
        // Print title
        println!(
            "{}",
            color(
                &format!(
                    "Randomly generated word{s}:",
                    s = if words.len() == 1 { "" } else { "s" }
                ),
                Color::Blue,
                !args.no_color,
            )
        );

        // Print words
        for word in words {
            println!(" {} {}", color("-", Color::Cyan, !args.no_color), word);
        }
    }

    Ok(())
}
