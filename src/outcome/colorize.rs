pub mod colors {
    pub use colorful::Color::{Blue, Cyan, Green, Magenta, Red, Yellow};
}

/// Color text with `colorful::Color`, conditionally
pub fn color(text: &str, color: colorful::Color, do_color: bool) -> String {
    if do_color {
        colorful::Colorful::color(text, color).to_string()
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{color, colors::*};

    #[test]
    fn it_works() {
        assert_eq!(color("red", Red, false), "red");

        assert_eq!(color("red", Red, true), "\x1b[38;5;1mred\x1b[0m");
    }
}
