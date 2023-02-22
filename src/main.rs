mod args;

use std::fs;

use clap::Parser;
use stilo::{style, Style};

use phonet::{
    draft::{Message::Test, TestDraft},
    get_min_filename, Draft,
};

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

    // Add 'phonet' file extension if file argument ends with a period
    let filename = if args.file.ends_with('.') {
        args.file + "phonet"
    } else {
        args.file
    };

    // Read file
    let file = fs::read_to_string(&filename).expect("Could not read phonet file");

    // Parse file
    let mut draft = try_this!(Draft::from(&file));

    // Use custom CLI tests if given
    if !args.tests.is_empty() {
        draft.messages = args
            .tests
            .iter()
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
            get_min_filename(&filename),
            draft.minify(args.with_tests).expect("Failed to minify"),
        )
        .expect("Could not write minified file");
    }

    // Run tests and display
    draft.run().display(args.display_level, !args.no_color);

    // Generate and display words
    if let Some(count) = args.generate {
        // Default count to 1 word
        let count = count.unwrap_or(1);

        // Min and max length
        let min = args.generate_min_len.unwrap_or(3);
        let max = args.generate_max_len.unwrap_or(14);
        // Ensure min and max are not invalid
        let max = max.max(min);
        let min = min.min(max);

        // Generate words
        let mut words = try_this!(draft.generator(min..=max));

        // Print title
        println!(
            "{}",
            color("Randomly generated words:", style!(Blue), !args.no_color,)
        );

        // Print words
        for _ in 0..count {
            let word = words.next();

            println!(
                " {} {}",
                color("-", style!(Cyan), !args.no_color),
                color(&word, style!(-italic), !args.no_color)
            );
        }
    }

    Ok(())
}

/// Use `stilo::Color` to format text only if `do_color` is true
fn color(text: &str, style: Style, do_color: bool) -> String {
    if do_color {
        style.format(text)
    } else {
        text.into()
    }
}
