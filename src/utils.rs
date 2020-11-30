pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    // Find the index of the first non-digit character
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or_else(|| s.len()); // this is a lambda func that takes 0 args

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    // Check if the first char is an operator or panic
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Unknown operator"),
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10*20"), ("*20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_all_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+2"), ("2", "+"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-2"), ("2", "-"));
    }
    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*2"), ("2", "*"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/2"), ("2", "/"));
    }
}
