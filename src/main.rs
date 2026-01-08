use anyhow::{Context, Result};
use hex;

const USAGE: &str = "Usage: hex2string <hex>
Convert hexadecimal string to normal string.";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }
    match run(&args[1]) {
        Ok(string) => println!("{}", string),
        Err(e) => {
            eprintln!("ERROR {:#}", e);
            std::process::exit(1);
        }
    }
}

/// Describe function here
fn run(hex: &str) -> Result<String> {
    let bytes = hex::decode(hex)
        .with_context(|| format!("failed to convert hex to bytes"))?;
    let string = String::from_utf8(bytes)
        .with_context(|| format!("failed to convert bytes to string"))?;
    Ok(string)
}
