use std::usize;

use bitvec::prelude::*;

use crate::version::{num_to_bitvec, pad_then_append};

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

fn enocde_numeric(information: &str) -> BitVec {
    let mut bitvec = BitVec::<usize, Lsb0>::new();

    for i in (0..information.len()).step_by(3) {
        if i + 2 < information.len() {
            bitvec.append(&mut str_to_bitvec(&information[i..i + 2], 10));
        } else if i + 1 < information.len() {
            bitvec.append(&mut str_to_bitvec(&information[i..i + 2], 7));
        } else {
            bitvec.append(&mut str_to_bitvec(&information[i..i + 2], 4));
        }
    }
    bitvec
}

fn str_to_bitvec(small_str: &str, total_bits: usize) -> BitVec {
    let mut bitvec = BitVec::new();
    for c in small_str.chars() {
        let mut temp_bitvec = num_to_bitvec(c.to_digit(10).unwrap() as usize);
        let mut zeropad = bitvec![0; total_bits - temp_bitvec.len()];
        bitvec.append(&mut zeropad);
        bitvec.append(&mut temp_bitvec);
    }
    bitvec
}

fn enocde_alphanumeric(information: &str) -> BitVec {
    let mut bitvec = BitVec::new();
    for index in (0..information.len()).step_by(2) {
        if index + 1 < information.len() {
            let temp_bitvec = num_to_bitvec(
                convert_alphanumeric(information.chars().nth(index).unwrap()) * 45
                    + convert_alphanumeric(information.chars().nth(index + 1).unwrap()),
            );
            pad_then_append(11, &mut bitvec, temp_bitvec);
        } else {
            let mut temp_bitvec =
                num_to_bitvec(convert_alphanumeric(information.chars().nth(index).unwrap()) * 45);
            pad_then_append(6, &mut bitvec, temp_bitvec);
        }
    }
    bitvec
}

fn enocde_byte(information: &str) -> BitVec {
    //let str_raw = information.parse::<usize>().unwrap();
    num_to_bitvec(information.parse::<usize>().unwrap())
}

fn convert_alphanumeric(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 18,
        'J' => 19,
        'K' => 20,
        'L' => 21,
        'M' => 22,
        'N' => 23,
        'O' => 24,
        'P' => 25,
        'Q' => 26,
        'R' => 27,
        'S' => 28,
        'T' => 29,
        'U' => 30,
        'V' => 31,
        'W' => 32,
        'X' => 33,
        'Y' => 34,
        'Z' => 35,
        ' ' => 36,
        '$' => 37,
        '%' => 38,
        '*' => 39,
        '+' => 40,
        '-' => 41,
        '.' => 42,
        '/' => 43,
        ':' => 44,
        _ => panic!("Unsupported alphanumeric '{}'", c),
    }
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
