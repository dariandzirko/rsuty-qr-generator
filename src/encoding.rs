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
        let num = c.to_digit(10).unwrap() as usize;
        let bitvec_size_raw = num.view_bits::<Lsb0>();
        let bitvec_size_bits = bitvec_size_raw
            .iter_ones()
            .last()
            .unwrap_or(bitvec::mem::bits_of::<usize>() - 1);

        let mut temp_bitvec = bitvec_size_raw[..=bitvec_size_bits].to_bitvec();
        let mut zeropad = bitvec![0; total_bits - temp_bitvec.len()];
        bitvec.append(&mut zeropad);
        bitvec.append(&mut temp_bitvec);
    }
    bitvec
}

fn enocde_alphanumeric(information: &str) {}
fn enocde_byte(information: &str) {}

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
