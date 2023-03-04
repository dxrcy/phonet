use stilo::Style;

// use phonet::Draft;

/// Unwrap the `Ok` value of a `Result`, or exit with a stringified `Error`
macro_rules! try_or_throw {
    ( $result: expr ) => {{
        match $result {
            Ok(value) => value,

            Err(err) => {
                return Err(err.to_string());
            }
        }
    }};
}

/// Returns an `Err` with a formatted `String`
macro_rules! throw {
    () => {
        return Err(String::new())
    };
    ( $str: literal ) => {
        return Err($str.to_string())
    };
    ( $str: literal, $( $arg: tt ),* ) => {
        return Err(format!($str, $( $arg )*))
    };
}

/// Use `stilo::Color` to format text only if `do_color` is true
pub fn _color(text: &str, style: Style, do_color: bool) -> String {
    if do_color {
        style.format(text)
    } else {
        text.into()
    }
}

/// Format filename when used with shorthand
pub fn format_filename(name: String) -> String {
    // Add 'phonet' file extension if file argument ends with a period
    if name.ends_with('.') {
        return name + "phonet";
    }

    // Add 'phonet' file name if file argument ends with a slash (is a directory)
    if name.ends_with('/') || name.ends_with('\\') {
        return name + "phonet";
    }

    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_filename_works() {
        assert_eq!(format_filename("phonet".to_string()), "phonet");
        assert_eq!(format_filename("abc.phonet".to_string()), "abc.phonet");
        assert_eq!(format_filename("abc.".to_string()), "abc.phonet");
        assert_eq!(format_filename("src/".to_string()), "src/phonet");
        assert_eq!(format_filename("src".to_string()), "src");
        assert_eq!(format_filename("src/abc.".to_string()), "src/abc.phonet");
    }
}
