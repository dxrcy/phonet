#[cfg(test)]
mod tests;

use std::ops::Range;

use fancy_regex_macro::regex;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::{
    draft::{replace_classes, Classes, Draft, Rule},
    error::Error,
    outcome::{validate_test, Validity::*},
};

impl Draft {
    /// Generate random words that is valid against rules
    pub fn generate(&self, count: usize, length: Range<usize>) -> Result<Vec<String>, Error> {
        let letters = get_letters(&self.raw_classes)?;
        let mut words = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Loops until it finds a valid word
            words.push(generate_valid_word(
                &letters,
                &self.rules,
                &length,
                &mut rng,
            ));
        }

        Ok(words)
    }
}

/// Generate a random word, with a random length, that is valid against rules
fn generate_valid_word(
    letters: &str,
    rules: &[Rule],
    length: &Range<usize>,
    rng: &mut ThreadRng,
) -> String {
    loop {
        let word = random_word(letters, rng.gen_range(length.clone()), rng);

        if matches!(validate_test(&word, rules), Valid) {
            return word;
        }
    }
}

/// Generate a random word, with a set length, that may not be valid
fn random_word(letters: &str, length: usize, rng: &mut ThreadRng) -> String {
    let chars: Vec<char> = letters.chars().collect();
    let mut word = String::new();

    for _ in 0..length {
        word.push(*chars.choose(rng).unwrap());
    }

    word
}

/// Get letters of 'any' class, without regex symbols
fn get_letters(classes: &Classes) -> Result<String, Error> {
    Ok(remove_regex_symbols(&get_any_class(classes)?))
}

/// Remove most regex symbols from a string, such as `[`, `]`, `|`, ect
fn remove_regex_symbols(pattern: &str) -> String {
    regex!(r"[\[\]|?*+{}():]")
        .replace_all(pattern, "")
        .to_string()
}

/// Get 'any' class (named `_`) from classes
fn get_any_class(classes: &Classes) -> Result<String, Error> {
    // Get class
    let Some((pattern, line)) = classes.get("_") else {
        return Err(Error::MissingAnyClass);
    };

    // Replace inner classes
    let pattern = replace_classes(pattern, classes, *line)?;

    Ok(pattern)
}
