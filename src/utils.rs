pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    // Find index of first character thats not accepted
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

// This is for when take_while is required to extract something or give error
pub(crate) fn take_while_req(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_req(|c| c.is_ascii_digit(), s, "Expected digits".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_whitespace_req(s: &str) -> Result<(&str, &str), String> {
    take_while_req(|c| c == ' ', s, "Expected a space".to_string())
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    // If the input doesnt start with an alphabet, it cant be an identifier
    // and we dont consume anything
    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("Expected identifier".to_string())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("Expected {}", starting_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10*20"), Ok(("*20", "10")));
    }

    #[test]
    fn do_not_extract_anything_when_input_invalid() {
        assert_eq!(extract_digits("abc"), Err("Expected digits".to_string()));
    }

    #[test]
    fn extract_all_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }

    fn dont_extract_spaces_req_when_input_no_space() {
        assert_eq!(
            extract_whitespace_req("abc"),
            Err("Expected a space".to_string())
        )
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abc stop"), Ok((" stop", "abc")));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foo1()"), Ok(("()", "foo1")));
    }

    #[test]
    fn dont_extract_ident_starting_with_number() {
        assert_eq!(
            extract_ident("12ronak"),
            Err("Expected identifier".to_string())
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}
