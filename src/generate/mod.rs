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

/// Generator for random valid words
pub struct Generator {
    rng: ThreadRng,
    /// Word length range
    length: Range<usize>,
    /// Letters from 'any' class
    letters: String,
    /// Rules to test against
    rules: Vec<Rule>,
}

impl Generator {
    /// Create a new word `Generator` from a `Draft`, with a word length range
    pub fn new(draft: &Draft, length: Range<usize>) -> Result<Self, Error> {
        let letters = get_letters(&draft.raw_classes)?;

        Ok(Self {
            rng: rand::thread_rng(),
            length,
            letters,
            rules: draft.rules.clone(),
        })
    }

    /// Generate a random word, with a random length, that is valid against rules
    pub fn next(&mut self) -> String {
        loop {
            let word = random_word(
                &self.letters,
                self.rng.gen_range(self.length.clone()),
                &mut self.rng,
            );

            if matches!(validate_test(&word, &self.rules), Valid) {
                return word;
            }
        }
    }
}

impl Draft {
    /// Create a new word `Generator` from a `Draft`, with a word length range
    pub fn generator(&self, length: Range<usize>) -> Result<Generator, Error> {
        Generator::new(self, length)
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
