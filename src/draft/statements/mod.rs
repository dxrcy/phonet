#[cfg(test)]
mod tests;

/// Split file into list of statements
pub(super) fn split_statements(file: &str) -> Vec<(String, usize)> {
    // Vector of statements
    // Multiline statements are treated as single line, with linebreaks removed
    let mut statements = vec![];

    // Building single line (and multiline, before '&' character)
    let mut build_line = String::new();
    // Building multiline, optional
    let mut build_multiline: Option<(String, usize)> = None;

    // Canon line number of statement
    // Multiline uses line number of beginning of statement
    let mut current_line_number = 1;

    // Loop characters of file
    for ch in file.chars() {
        match ch {
            // Newline or semicolon without multiline
            '\n' | ';' if build_multiline.is_none() => {
                // If single line is not empty
                if !build_line.is_empty() {
                    // Push single line to statement
                    statements.push((build_line, current_line_number));
                    // Reset single line
                    build_line = String::new();
                }
            }

            // Newline with multiline - Ignore
            '\n' => (),

            // Semicolon with multiline
            ';' => {
                // Multiline is active
                // Unwrap should not fail due to above match guard
                let (multiline, number) = build_multiline.unwrap();

                // Add multiline to single line, without linebreaks
                build_line.push_str(&multiline);
                // Reset multiline
                build_multiline = None;

                // If single line (including multiline) is not empty
                // This mirrors the statement in arm of '\n' match, above
                if !build_line.is_empty() {
                    // Push single line to statement
                    statements.push((build_line, number));
                    // Reset single line
                    build_line = String::new();
                }
            }

            // Start multiline
            '&' => match &mut build_multiline {
                // Multiline is not already active
                None => {
                    // Start multiline, with current line number
                    build_multiline = Some((String::new(), current_line_number))
                }

                // Multiline is already active
                Some(_) => {
                    // Add '&' character to single line build
                    build_line.push(ch);
                }
            },

            // Add other character to single line build
            _ => build_line.push(ch),
        }

        // Increase canon line number
        if ch == '\n' {
            current_line_number += 1;
        }
    }

    // Get line number of statement
    let start_line_number = match build_multiline {
        // Multiline is not active - Use current line number (last line)
        None => current_line_number,
        // Multiline is active
        Some((multiline, number)) => {
            // Add multiline to single line, without linebreaks
            build_line.push_str(&multiline);
            // Use line number of beginning of statement
            number
        }
    };

    // If single line (including multiline) is not empty
    if !build_line.is_empty() {
        // Push single line to statement
        statements.push((build_line, start_line_number));
    }

    statements
}
