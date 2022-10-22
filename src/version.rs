use bitvec::prelude::*;
use range_check::Check;

struct ErrorCorrectionLevel {
    name: string,
    value: usize,
}

impl ErrorCorrectionLevel {
    fn get_value(&self) {
        match (self.name) {
            "L" => self.value = 0,
            "M" => self.value = 1,
            "Q" => self.value = 2,
            "H" => self.value = 3,
            //_ => Error and get mad at user
        }
    }
}

//Should this return an option
//Also is there a better way to do this, then using pretty much treeman's method
fn determine_version(
    &str: information,
    ErrorCorrectionLevel: error_correction_level,
    EncodingMode: encoding,
) -> usize {
    let information_len = information.size();
    for version in 0..40 {
        if information_len
            <= CHARACTER_CAPACITIES[version][error_correction_level.value][encoding::value]
        {
            return version + 1;
        }
    }
    return None;
}

fn character_count_indicator(
    EncodingMode: encoding,
    usize: version,
    usize: information_len,
) -> bitvec {
    let mut bitvec_size = 0;
    if version.range_check(1, 9) {
        match encoding::value {
            0 => bitvec_size = 10,
            1 => bitvec_size = 9,
            2 => bitvec_size = 8,
            //_ => Blow up
        }
    } else if version.range_check(10, 26) {
        match encoding::value {
            0 => bitvec_size = 12,
            1 => bitvec_size = 11,
            2 => bitvec_size = 16,
            //_ => Blow up
        }
    } else if version.range_check(27, 40) {
        match encoding::value {
            0 => bitvec_size = 14,
            1 => bitvec_size = 13,
            2 => bitvec_size = 16,
            //_ => Blow up
        }
    }

    //I want a bitvec that is size bitvec_size but contains the properly zero padded information that is the information_len
    //ex. bitvec_size = 9 | information_len = 11 ---> return 000001101; with the zeros being important
}
