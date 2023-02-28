#[cfg(test)]
mod tests;

use stilo::{style, stylize, Style};

use super::Outcome;
use crate::{
    outcome::{FailKind::*, Message::*, Note, PassStatus::*, TestOutcome},
    DisplayLevel::{self, *},
};

/// Use `stilo::Color` to format text only if `do_color` is true
fn color(text: &str, style: Style, do_color: bool) -> String {
    if do_color {
        style.format(text)
    } else {
        text.into()
    }
}

impl Outcome {
    /// Get maximum length of all test words
    ///
    /// For printing to output
    pub fn max_word_len(&self, display_level: DisplayLevel) -> usize {
        self.messages
            .iter()
            .map(|msg| match msg {
                // Test - Check display level
                Test(TestOutcome { word, status, .. }) => match display_level {
                    // Always include
                    ShowAll => word.chars().count(),
                    // Only include if failed
                    IgnorePasses | OnlyFails if status.is_fail() => word.chars().count(),
                    // Don't include
                    _ => 0,
                },

                Info(_) => 0,
            })
            .max()
            .unwrap_or(0)
    }

    /// Get count of tests in list
    pub fn test_count(&self) -> usize {
        self.messages.iter().filter(|item| item.is_test()).count()
    }

    /// Display results to standard output
    ///
    /// This can be implemented manually
    pub fn display(&self, display_level: DisplayLevel, do_color: bool) {
        self.display_with(&mut std::io::stdout(), display_level, do_color)
            .expect("Could not write to stdout");
    }

    /// Display results, using custom output
    ///
    /// This can be implemented manually
    pub fn display_with(
        &self,
        out: &mut dyn std::io::Write,
        display_level: DisplayLevel,
        do_color: bool,
    ) -> Result<(), std::io::Error> {
        let test_count = self.test_count();

        // No tests
        if self.test_count() == 0 {
            writeln!(out, "{}", color("No tests ran", style!(Yellow), do_color))?;
            return Ok(());
        }

        // Initial print
        writeln!(
            out,
            "{}",
            color(
                &format!(
                    "Running {count} test{s}...",
                    count = test_count,
                    s = pluralize(test_count)
                ),
                style!(Yellow),
                do_color
            ),
        )?;

        // Get maximum length of all test words
        let max_word_len = self.max_word_len(display_level);

        // Loop result list
        for msg in &self.messages {
            match msg {
                // Display note
                Info(Note(note)) => match display_level {
                    // Always show - Print note
                    ShowAll | IgnorePasses => {
                        writeln!(out, "{}", color(note, style!(Blue), do_color))?;
                    }

                    // Else skip
                    _ => (),
                },

                // Display test
                Test(TestOutcome {
                    word,
                    intent,
                    status,
                }) => {
                    // Skip if not required by display level
                    match display_level {
                        // Always show
                        ShowAll => (),
                        // Only show if failed
                        IgnorePasses | OnlyFails if status.is_fail() => (),
                        // Else skip
                        _ => continue,
                    };

                    // Format reason with style
                    let reason = match status {
                        Pass => String::new(),
                        Fail(ShouldBeInvalid) => {
                            color("Valid, but should be invalid", style!(Yellow), do_color)
                        }
                        Fail(NoReasonGiven) => stylize!("No reason given", -italic),
                        Fail(CustomReason(Note(reason))) => String::from(reason),
                    };

                    // Display test outcome
                    writeln!(
                        out,
                        "  {intent} {word}{space}  {status} {reason}",
                        // Intent
                        intent = if *intent {
                            color("✔", style!(Cyan), do_color)
                        } else {
                            color("✘", style!(Magenta), do_color)
                        },
                        // Spacing after word
                        space = " ".repeat(max_word_len - word.chars().count()),
                        // Status of test
                        status = if status.is_pass() {
                            color(
                                "pass",
                                // Dim if some failed
                                if self.fail_count == 0 {
                                    style!(Green)
                                } else {
                                    style!(Green dim)
                                },
                                do_color,
                            )
                        } else {
                            color("FAIL", style!(Red bold), do_color)
                        },
                    )?;
                }
            }
        }

        // Final print
        if self.fail_count == 0 {
            // All passed
            writeln!(
                out,
                "{}",
                color("All tests pass!", style!(Green bold), do_color)
            )?;
        } else {
            // Some tests failed
            writeln!(
                out,
                "{}",
                color(
                    &format!(
                        "{count} test{s} failed",
                        count = self.fail_count,
                        s = pluralize(self.fail_count),
                    ),
                    style!(Red bold),
                    do_color
                )
            )?;
        };

        Ok(())
    }
}

/// Returns `"s"` if number does not equal `1`, else a blank string
fn pluralize(number: usize) -> &'static str {
    if number == 1 {
        ""
    } else {
        "s"
    }
}
