mod capacities;
mod encoding;
mod version;

fn main() {
    println!("Hello, world!");

    let info = "HELLO WORLD";
    let eclevel = version::ErrorCorrectionLevel::new("Q");
    let enc = encoding::determine_encoding(info);
    let ver = version::determine_version(info, eclevel, &enc);
    let char_count = version::character_count_indicator(&enc, ver, info.len());
    println!("char_count{:?}", char_count);
}
