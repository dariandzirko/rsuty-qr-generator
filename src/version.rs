use bitvec::prelude::*;
use range_check::*;

use crate::capacities::CHARACTER_CAPACITIES;
use crate::encoding::EncodingMode;

pub struct ErrorCorrectionLevel {
    value: usize,
}

impl ErrorCorrectionLevel {
    pub fn new(name: &str) -> Self {
        Self {
            value: Self::get_value(name),
        }
    }

    fn get_value(name: &str) -> usize {
        match name {
            "L" => return 0,
            "M" => return 1,
            "Q" => return 2,
            "H" => return 3,
            _ => return 100000,
        }
    }
}

//Should this return an option
//Also is there a better way to do this, then using pretty much treeman's method
pub fn determine_version(
    information: &str,
    error_correction_level: ErrorCorrectionLevel,
    encoding: &EncodingMode,
) -> usize {
    let information_len = information.len();
    for version in 0..40 {
        if information_len
            <= CHARACTER_CAPACITIES[version][error_correction_level.value][encoding.value()]
        {
            return version + 1;
        }
    }
    return 0;
}

pub fn character_count_indicator(
    encoding: &EncodingMode,
    version: usize,
    information_len: usize,
) -> BitVec {
    let mut bitvec_size = 0;
    if version.check_range(1..9).is_ok() {
        match encoding {
            EncodingMode::Numeric => bitvec_size = 10,
            EncodingMode::Alphanumeric => bitvec_size = 9,
            EncodingMode::Byte => bitvec_size = 8,
        }
    } else if version.check_range(10..26).is_ok() {
        match encoding {
            EncodingMode::Numeric => bitvec_size = 12,
            EncodingMode::Alphanumeric => bitvec_size = 11,
            EncodingMode::Byte => bitvec_size = 16,
        }
    } else if version.check_range(27..40).is_ok() {
        match encoding {
            EncodingMode::Numeric => bitvec_size = 14,
            EncodingMode::Alphanumeric => bitvec_size = 13,
            EncodingMode::Byte => bitvec_size = 16,
        }
    }

    let encoding_bitvec = encoding.mode_indicator();
    println!("{:?}", encoding_bitvec);
    //encoding_bitvec.append(information_len);

    return encoding_bitvec;
    //I want a bitvec that is size bitvec_size but contains the properly zero padded information that is the information_len
    //ex. bitvec_size = 9 | information_len = 11 ---> return 000001101; with the zeros being important
}
