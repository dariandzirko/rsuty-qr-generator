mod encoding;
mod version;

fn main() {
    println!("Hello, world!");

    let info = "HELLO WORLD";
    let eclevel = ErrorCorrectionLevel::new("Q");
    let enc = encoding::determine_encoding(info);
    let ver = determine_version(info);
}
