# TODO

## Code

### Documentation

- Add docs to readme !!!
- Add doc comments !!!

### Tests

- More unit tests
- Add `display` to integration tests

### Clean code

- Only use `clap` dependency for binary
- Move contents of `draft/types.rs` to `draft/mod.rs`
- - For parity with `outcome`
- - Move `Draft::parse` method to `draft/parse`

### API

- Use line numbers in class and rule parsing

## Minor / Future Features

- Use config struct for `display` function
- - Holds `DisplayLevel` and `do_color`

- Print generated words one by one

- Conditional items? (rules, tests, notes, ect)
