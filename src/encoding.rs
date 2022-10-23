use bitvec::prelude::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
    //Kanji(usize) = 3, This is for more difficult character sets
}

impl EncodingMode {
    pub fn mode_indicator(&self) -> BitVec {
        match self {
            EncodingMode::Numeric => return bitvec![0, 0, 0, 1],
            EncodingMode::Alphanumeric => return bitvec![0, 0, 1, 0],
            EncodingMode::Byte => return bitvec![0, 1, 0, 0],
            //_ => Blow up
        }
    }
    pub fn value(&self) -> usize {
        match self {
            EncodingMode::Numeric => return 0,
            EncodingMode::Alphanumeric => return 1,
            EncodingMode::Byte => return 2,
            //_ => Blow up
        }
    }
}

pub fn determine_encoding(information: &str) -> EncodingMode {
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
    use crate::encoding::{determine_encoding, EncodingMode};

    #[test]
    fn test_determine_encoding() {
        assert_eq!(determine_encoding("123"), EncodingMode::Numeric);
        assert_eq!(determine_encoding("123ABC"), EncodingMode::Alphanumeric);
        assert_eq!(determine_encoding("123abc"), EncodingMode::Byte);
    }
}
