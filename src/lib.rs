use std::str::EncodeUtf16;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

fn determine_encoding(information: &str) -> EncodingMode {
    let mut mode = EncodingMode::Numeric;
    for char in information.chars() {
        if char.is_ascii_uppercase() {
            mode = std::cmp::max(mode, EncodingMode::Alphanumeric);
        } else if char.is_ascii_lowercase() {
            mode = std::cmp::max(mode, EncodingMode::Byte);
        }
    }

    return mode;
}

#[cfg(test)]
mod tests {
    use crate::{determine_encoding, EncodingMode};

    #[test]
    fn test_determine_encoding() {
        assert_eq!(determine_encoding("123"), EncodingMode::Numeric);
        assert_eq!(determine_encoding("123ABC"), EncodingMode::Alphanumeric);
        assert_eq!(determine_encoding("123abc"), EncodingMode::Byte);
    }
}
