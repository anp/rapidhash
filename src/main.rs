use std::io::Read;

/// Command-line tool for rapidhash.
///
/// Rapidhash produces a `u64` hash value, and terminal output is a decimal string of the hash
/// value.
///
/// # Install
/// ```shell
/// cargo install rapidhash
/// ```
///
/// # Usage
/// Reading stdin:
/// ```shell
/// echo "example" | rapidhash
/// 8543579700415218186
/// ```
///
/// Reading file:
/// ```bash
/// rapidhash example.txt
/// 8543579700415218186
/// ```
pub fn main() {
    let hash_arg = std::env::args().nth(1);

    let buffer = match hash_arg {
        None => {
            let mut buffer = Vec::with_capacity(1024);
            std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");
            buffer
        }
        Some(filename) => {
            std::fs::read(filename).expect("Could not load file.")
        }
    };

    let hash = rapidhash::rapidhash(&buffer);
    println!("{hash}");
}
