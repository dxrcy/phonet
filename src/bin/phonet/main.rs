mod args;
#[macro_use]
mod utils;

use std::{fs, path::Path};

use clap::Parser;
use stilo::style;

use phonet::{
    draft::{Message::Test, TestDraft},
    get_min_filename, DisplayLevel, Draft,
};

use crate::args::Args;
use crate::utils::{color, format_filename};

fn main() -> Result<(), String> {
    let args = Args::parse();
    let do_color = !args.no_color;

    // Format filename (expand shorthand)
    let filename = format_filename(args.file);

    // File must exist
    let path = Path::new(&filename);
    if !path.exists() {
        throw!("File not found '{}'", filename);
    }
    // File must not be a directory
    if path.is_dir() {
        throw!("Filename is a directory '{}'. Tip: End filename with '/' to use 'phonet' file in that directory", filename);
    }

    // Read file
    let file = match fs::read_to_string(&filename) {
        Ok(x) => x,
        Err(err) => throw!("Failed to read file: `{:?}`", err),
    };

    // Parse file
    let mut draft = try_or_throw!(Draft::from(&file));

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
        let minified = try_or_throw!(draft.minify(args.with_tests));

        // Write file
        if let Err(err) = fs::write(get_min_filename(&filename), minified) {
            throw!("Failed to write minified file: `{:?}`", err);
        };
    }

    // Use display level
    let display_level = if args.quiet {
        DisplayLevel::OnlyFails
    } else {
        DisplayLevel::ShowAll
    };

    // Print name before outcome, if given
    if let Some(name) = &draft.name {
        println!("{}", color(name, style!(Cyan italic), do_color));
    }

    // Run tests and display
    draft.run().display(display_level, do_color);

    // Generate and display words
    if let Some(count) = args.generate {
        // Default count to 1 word
        let count = count.unwrap_or(1);

        // Min and max length
        let min = args.generate_min_len;
        let max = args.generate_max_len;
        // Ensure min and max are not invalid
        let max = max.max(min);
        let min = min.min(max);

        // Generate words
        let mut words = try_or_throw!(draft.generator(min..=max));

        // Print title
        println!(
            "{}",
            color("Randomly generated words:", style!(Blue), do_color,)
        );

        // Print words
        for _ in 0..count {
            let word = words.next();

            println!(
                " {} {}",
                color("-", style!(Cyan), do_color),
                color(&word, style!(-italic), do_color)
            );
        }
    }

    Ok(())
}
