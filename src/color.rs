pub mod colors {
    pub use colorful::Color::{Blue, Cyan, Green, Magenta, Red, Yellow};
}

/// Color text with `colorful::Color`, conditionally
pub fn colorize(text: &str, color: colorful::Color, do_color: bool) -> String {
    if do_color {
        colorful::Colorful::color(text, color).to_string()
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{colorize, colors::*};

    #[test]
    fn it_works() {
        assert_eq!(colorize("red", Red, false), "red");

        assert_eq!(colorize("red", Red, true), "\x1b[38;5;1mred\x1b[0m");
    }
}
