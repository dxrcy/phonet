use super::{
    colorize::{color, colors::*},
    Outcome,
};
use crate::{
    DisplayLevel::{self, *},
    FailKind::*,
    Message::*,
    Note,
    PassStatus::{self, *},
    TestOutcome,
};

impl Outcome {
    /// Get maximum length of all test words
    ///
    /// TODO Clean this
    /// TODO Test !
    pub fn max_word_len(&self, display_level: DisplayLevel) -> usize {
        self.list
            .iter()
            .map(|x| match x {
                // Test - Check display level
                Test(TestOutcome { word, status, .. }) => match display_level {
                    // Always include
                    ShowAll => word.chars().count(),
                    // Only include if failed
                    IgnorePasses | OnlyFails if matches!(status, PassStatus::Fail(_)) => {
                        word.chars().count()
                    }
                    // Don't include
                    _ => 0,
                },

                Info(_) => 0,
            })
            .max()
            // Default value
            .unwrap_or(10)
    }

    /// Get count of tests in list
    pub fn test_count(&self) -> usize {
        self.list
            .iter()
            .filter(|item| matches!(item, Test(_)))
            .count()
    }

    /// Display results to standard output
    ///
    /// This can be implemented manually
    ///
    /// TODO Use config struct for `display_level` and `do_color` ??
    pub fn display(self, display_level: DisplayLevel, do_color: bool) {
        self.display_with(&mut std::io::stdout(), display_level, do_color)
            .expect("Could not write to stdout");
    }

    /// Display results, using custom output
    ///
    /// This can be implemented manually
    pub fn display_with(
        self,
        out: &mut dyn std::io::Write,
        display_level: DisplayLevel,
        do_color: bool,
    ) -> Result<(), std::io::Error> {
        let test_count = self.test_count();

        // No tests
        if self.test_count() == 0 {
            writeln!(out, "{}", color("No tests ran.", Yellow, do_color))?;
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
                Yellow,
                do_color
            )
        )?;

        // Get maximum length of all test words
        #[allow(unused_variables)]
        let max_word_len = self.max_word_len(display_level);

        // Loop result list
        for msg in &self.list {
            // ? match (msg, display_level) {
            match msg {
                // Display note
                Info(Note(note)) => match display_level {
                    // Always show - Print note
                    ShowAll | IgnorePasses => {
                        writeln!(out, "{}", color(note, Blue, do_color))?;
                    }

                    // Else skip
                    _ => (),
                },

                // Display test
                #[allow(unused_variables)]
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

                    // Colored text for `ShouldBeInvalid` fail
                    let should_be_invalid = color("Valid, but should be invalid", Yellow, do_color);

                    // Format reason
                    let reason = match status {
                        Pass => "",
                        Fail(ShouldBeInvalid) => &should_be_invalid,
                        Fail(NoReasonGiven) => "No reason given",
                        Fail(CustomReason(Note(reason))) => reason,
                    };

                    // Display test outcome
                    writeln!(
                        out,
                        "  {intent} {word}{space}  {status} {reason}",
                        // Intent
                        intent = if *intent {
                            color("✔", Cyan, do_color)
                        } else {
                            color("✗", Magenta, do_color)
                        },
                        // Word
                        word = word,
                        // Spacing after word
                        space = " ".repeat(max_word_len - word.chars().count()),
                        // Status of test
                        status = if status.is_pass() {
                            color("pass", Green, do_color)
                        } else {
                            color("FAIL", Red, do_color)
                        },
                        // Reason (if failed)
                        reason = reason,
                    )?;
                }
            }
        }

        // Final print
        if self.fail_count == 0 {
            // All passed
            writeln!(out, "{}", color("All tests pass!", Green, do_color))?;
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
                    Red,
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