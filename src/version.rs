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
