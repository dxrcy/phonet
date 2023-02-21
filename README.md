# Phoner

_Phonet_ is a CLI tool and library to validate phonotactic patterns for constructed languages.
It is compatible with either romanization and phonetic transcription.
Words can be randomly generated (see [Argument Syntax](#argument-syntax)).

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phonet-syntax)

> Formerly named 'Phoner'

# Usage

This project can be used as a rust library crate, or as a binary executable.

## Binary use

[Download latest version here](https://github.com/darccyy/phonet/releases/latest)

### Argument Syntax

```
Usage: phonet.exe [OPTIONS] [TESTS]...

Arguments:
  [TESTS]...
          Custom tests (Optional)

          This overrides all tests in the file

Options:
  -f, --file <FILE>
          Name and path of file to run and test

          If name ends with a period, the 'phonet' extension is implied

          Eg. `phonet -f ./myfile.phonet` or `phonet -f ./myfile.` (same result)

          [default: phonet]

  -d, --display-level <DISPLAY_LEVEL>
          What types of outputs to display

          Options can be single letter

          Eg. `phonet -d only-fails` or `phonet -do`

          [default: show-all]

          Possible values:
          - show-all:      Show everything: passed or failed tests, and notes
          - ignore-passes: Show failed tests and notes, but not passes
          - only-fails:    Show only failed tests, not passed tests or notes
          - hide-all:      Show nothing: not passed or failed tests, or notes

  -m, --minify
          Minify file and save

  -w, --with-tests
          Include tests in minified file

  -g, --generate [<GENERATE>]
          Generate random words

          Default count 1, specify with number

      --gmin <GENERATE_MIN_LEN>
          Set minimum length (inclusive) for generated words

          Use with the `--generate` or `-g` flag

          Note: This increases generation time exponentially

      --gmax <GENERATE_MAX_LEN>
          Set maximum length (inclusive) for generated words

          Use with the `--generate` or `-g` flag

  -n, --no-color
          Display output in default color

          Use for piping standard output to a file

  -h, --help
          Print help (see a summary with '-h')
```

### Example

```bash
# Runs ./phonet
phonet

# Runs ./phonet, with tests: 'some', 'tests' (overrides the tests in file)
phonet some tests

# Runs ./myfile.phonet
phonet -f myfile.phonet
phonet -f myfile.phonet some tests

# Runs ./phonet, only showing fails
phonet -df
# Alternatives:
phonet -d only-fails
phonet -d fails

# Runs ./phonet, and minifies to ./min.phonet without tests
phonet -m

# Runs ./myfile.phonet, without outputting any results, and minifies to ./myfile.min.phonet with tests
phonet -f myfile.phonet -dh -mw

# Runs ./phonet, and generates 1 random word
phonet -g

# Runs ./myfile.phonet, and generates 10 random words
phonet -g10 -f myfile.phonet

# Runs ./phonet, with no color, and writes output to ./phonet.txt
phonet -n > phonet.txt

# Runs ./myfile.phonet, with all test output hidden, and generates 3 random words with length 6-8, writes output to ./phonet.txt (with no color)
phonet -f myfile.phonet -ndh -g 3 --gmin 6 --gmax 8 > ./phonet.txt
```

### Create Alias / Path

Replace `<path_to_file>` with the directory of the downloaded binary.

#### Bash

Add alias in `.bashrc` in user directory

```bash
# ~/.bashrc
alias phonet="<path_to_file>/phonet.exe"
```

#### Powershell

Add to `$env:PATH`

```ps1
$env:Path = "$env:Path;<path_to_file>\phonet.exe"
```

## Library use

Add `phonet = "0.9.0"` to your `Crates.toml` file

- [Docs.rs](https://docs.rs/phonet/latest/phonet)
- [Crates.io](https://crates.io/crates/phonet)

### Short Example

```rs
use phonet::Draft;

fn main() {
    let file = std::fs::read_to_string("phonet").unwrap();

    // Parse draft
    Draft::from(&file).unwrap()
        // Run tests
        .run()
        // Display results
        .display(Default::default())
}
```

### Long Example

```rs
use std::fs;

use phonet::{
    draft::{Message::Test, TestDraft},
    get_min_filename, DisplayLevel, Draft,
};

fn main() {
    let filename = "myfile.phonet";

    // Read file
    let file = fs::read_to_string(filename).expect("Could not read phonet file");

    // Parse file
    let mut draft = Draft::from(&file).expect("Failed to parse file");

    // Add a custom test
    draft.messages.push(Test(TestDraft {
        intent: true,
        word: "taso".to_string(),
    }));

    // Minify file
    fs::write(
        get_min_filename(filename),
        draft.minify(false).expect("Failed to minify"),
    )
    .expect("Could not write minified file");

    // Run tests and display only failed tests
    draft.run().display(DisplayLevel::OnlyFails, true);

    // Generate 10 words, each between 5 and 8 in length, and print each
    println!("Randomly generated words:");
    for word in draft.generate(10, 5..8).expect("Failed to generate words") {
        println!(" - {}", word);
    }
}
```

# File syntax

A _Phonet_ file is used to define the rules, classes, and tests for the program.

The file should either be called `phonet`, or end in `.phonet`

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phonet-syntax)

## Statements

The syntax is a statements, each separated by a semicolon `;` or a linebreak.

Use a _Ampersand_ `&` to denote a multi-line statement. This can only be ended with a semicolon `;`.

Comments will end with a linebreak or a semicolon `;`.

All whitespace is ignored, except to separate words in [_tests_](#tests).

> Note! This will replace spaces in Regex as well! Use `\s` if you need a space

Each statement must begin with an operator:

- `#` _Hashtag_: A whole line comment. A linebreak (not a semicolon) ends the comment
- `$` _Dollar_: Define a [_class_](#classes)
- `+` **_Plus_** or `!` **_Bang_**: Define a [_rule_](#rule)
- `*` _Star_: Create a test [_note_](#notes), and define a _reason_ if a test fails
- `?` _Question_: Create a [_test_](#tests)
- `~` _Tilde_: Define the [_mode_](#mode) of the file

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_rules_](#rules) at runtime.

> **Note:** Angle brackets will not parse as class names directly after:
>
> - An opening round bracket and a question mark: `(?`
> - An opening round bracket, question mark, and letter 'P': `(?P`
> - A backslash and letter 'k': `\k`
>
> This is the syntax used for look-behinds and named groups

_Syntax:_

- `$` _Dollar_
- Name - Must be only characters from [a-zA-Z0-9_]
- `=` _Equals_
- Value - Regular Expression, may contain other _classes_ in angle brackets `<>` or `⟨⟩` (as with [_rules_](#rules))

The _'any'_ class, defined with `$_ = ...`, is used for random word generation.

_Example:_

```phonet
# Some consonants
$C = [ptksmn]

# Some vowels
$V = [iueoa]

# Only sibilant consonants
$C_s = [sz]
```

## Rules

Rules are Regular Expressions used to test if a word is valid.

Rules are defined with an _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ rule must be followed for a word to be valid
- A _negative_ rule must **not** be followed for a word to be valid

To use a [_class_](#classes), use the class name, surrounded by angle brackets `<>` or `⟨⟩`.

_Syntax:_

- `+` **_Plus_** or `!` **_Bang_** - Plus for _positive_ rule, Bang for _negative_ rule
- Pattern - Regular Expression, may contain [_classes_](#classes) in angle brackets `<>` or `⟨⟩`

_Example (with predefined [*classes*](#classes)):_

```phonet
# Must be (C)V syllable structure
+ ^ (<C>? <V>)+ $

# Must not have two vowels in a row
! <V>{2}
```

## Tests

Tests are checked against all rules, and the result is displayed in the output.

Tests are ran in the order of definition.

Like [_rules_](#rules), tests must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ test will pass if it is valid
- A _negative_ test will **fail** if it is valid

_Syntax:_

- `?` _Question mark_
- `+` **_Plus_** or `!` **_Bang_** - Plus for _positive_ test, Bang for _negative_ test
- Tests - A word, or multiple words separated by a space

_Example (with predefined [*rules*](#rules)):_

```phonet
# This should match, to pass
?+ taso
# This test should NOT match, to pass
?! tax
# Each word is a test, all should match to pass
?+ taso sato tasa
```

## Notes

Notes are printed to the terminal output, alongside tests.

They are used as a _reason_ for any proceeding rules, as an explanation if a test fails.

_Syntax:_

- `*` _Star_
- Text to print, and define reason as

_Example:_

```phonet
* Syllable structure
+ ^ (<C>? <V>)+ $

# This test will NOT match, however it SHOULD (due to the Plus), so it will FAIL, with the above note as the reason
?+ tasto

* Must not have two vowels in a row
! <V>{2}

?+ taso
```

## Mode

The mode of a _Phonet_ file can be one of these:

- _Romanized_: Using `<>` (not `⟨⟩`)
- _Broad transcription_: Using `//`
- _Narrow transcription_: Using `[]`

This can optionally be specified in a file, although it does not add any functionality.

_Syntax:_

- `~` _Tilde_
- `<.>`, `/./`, or `[.]` - Mode identifier, with `.` being any string, or blank

_Examples:_

```phonet
# Specify romanized mode (fish icon)
~<>
```

```phonet
# Specify broad transcription
~ / this is the mode /
```

## Examples

See the [examples](./examples/) folder for _Phonet_ file examples.

- [Good Syntax Example](./examples/example.phonet)
- [Toki Pona](./examples/tokipona.phonet)
<!-- - [Ivalingo](./examples/ivalingo.phonet) -->

## Recommended Syntax Patterns

These formatting tips are not required, but recommended to make the file easier to read.

1. Specify the mode at the very top of the file
2. Define all classes at the top of the file
   - Also define an [_'any'_ class](#classes) first, for word generation
3. Group related rules and tests, using a note
   - Define rules first, then positive tests, then negative tests
4. Indent rules and tests under note
   - Rules should use 1 intent, tests use 2

_Example (this is from [example.phonet](./examples/example.phonet)):_

```phonet
~<> ;# Mode (optional) - This file uses romanized letters

# Class definitions
$_ = ⟨C⟩ | ⟨V⟩        ;# Any / all letters (required for generating words)
$C = [ptkmnswjl]      ;# Consonants
$V = [aeiou]          ;# Vowels

* Invalid letters     ;# Note - Prints to standard output, and used as reason if test fails
  + ^ ⟨_⟩+ $          ;# Check that every letter is in the 'any' class
    ?+ taso
    ?! tyxo

* Examples of failing tests
    ?+ tyxo           ;# This test will fail - with the reason 'Invalid Letters' (above)
    ?! taso           ;# This test will fail, as a false positive

* Syllable structure
  + ^ ⟨V⟩? ( ⟨C⟩ ⟨V⟩ )+ $  ;# Check that word is Consonant + Vowel, repeating at least once
    ?+ taso kili ano atoso
    ?! taaso an

* Some more tests
    ?+ silo tila
    ?! akka axe

* No repeated letters
  ! (.)\1             ;# This is an unnamed back-reference
  ! (?<x> .) \k<x>    ;# This is a named back-reference (NOT a class)
    ?+ taso           ;# An example of multi-line statements on next line (comments cannot be on same line)
    ?! &
      taaso
      ttaso
    ;

* 2 tests *should* have failed!
```

![Phonet Icon](./icon.png)
