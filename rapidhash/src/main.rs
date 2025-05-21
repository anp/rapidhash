use std::io::{Read};

/// Command-line tool for rapidhash.
///
/// Rapidhash produces a `u64` hash value, and terminal output is a decimal string of the hash
/// value.
///
///
/// # Install
/// ```shell
/// cargo install rapidhash
/// ```
///
/// # Usage
///
/// ## Reading file
///
/// This will first check the metadata of the file to get the length, and then stream the file.
///
/// ```bash
/// rapidhash example.txt
/// 8543579700415218186
/// ```
///
/// ## Reading stdin
///
/// **NOTE:**
/// Because of how rapidhash is seeded using the data length, the length must be known at the start
/// of the stream. Therefore reading from stdin is not recommended, as it will cache the entire
/// input in memory before being able to hash it.
///
/// ```shell
/// echo "example" | rapidhash
/// 8543579700415218186
/// ```
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        println!("Usage: rapidhash [opts] [filename]");
        println!("[opts]");
        println!("  --v1  Use v1 hashing algorithm (must have v1 feature enabled)");
        println!("  --v2  Use v2 hashing algorithm (default; must have v2 or vlatest feature enabled)");
        println!("  --v3  Use v3 hashing algorithm (must have v3 feature enabled)");
        println!("[filename]");
        println!("  Providing a filename is optional and will read a file directly. Requires the std");
        println!("  feature to be enabled. Otherwise input is read from stdin.");
        println!("  ");
        println!("  Note that reading from stdin will buffer the whole input in memory before hashing,");
        println!("  while hashing a filename will load the file length from metadata and then stream");
        println!("  the file.");
        println!("Docs: https://github.com/hoxxep/rapidhash?tab=readme-ov-file#cli");
        return;
    }

    // file name is the first non-option argument
    let filename = args.iter().skip(1).filter(|a| !a.starts_with('-')).next();

    let v1 = args.iter().any(|a| a == "--v1");
    let v2 = args.iter().any(|a| a == "--v2") || !v1;
    if v1 && v2 {
        println!("Cannot use both --v1 and --v2 at the same time.");
        return;
    }

    let hash: u64 = match filename {
        None => {
            rapidhash::v3::rapidhash_file(std::io::stdin()).expect("Failed to read from stdin")

            // let mut buffer = Vec::with_capacity(1024);
            // std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");
            //
            // #[allow(unreachable_code)]
            // #[allow(unused_variables)]
            // match (v1, v2) {
            //     (true, false) => {
            //         #[cfg(not(feature = "v1"))]
            //         panic!("v1 feature is not enabled.");
            //
            //         #[cfg(feature = "v1")]
            //         rapidhash::v1::rapidhash(&buffer)
            //     }
            //     (false, true) => {
            //         #[cfg(not(any(feature = "v2", feature = "vlatest")))]
            //         panic!("v2 or vlatest feature is not enabled.");
            //
            //         #[cfg(feature = "v2")] {
            //             rapidhash::v2::rapidhash(&buffer)
            //         }
            //
            //         #[cfg(all(feature = "vlatest", not(feature = "v2")))] {
            //             rapidhash::rapidhash(&buffer)
            //         }
            //     }
            //     _ => unreachable!("Logic error."),
            // }
        }

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        Some(filename) => {
            #[cfg(not(feature = "std"))] {
                panic!("File reading is not supported without the `std` feature.");
            }

            let mut file = std::fs::File::open(filename).expect("Could not open file.");
            rapidhash::v3::rapidhash_file(&mut file).expect("Failed to hash file.")

            // #[cfg(feature = "std")]
            // match (v1, v2) {
            //     (true, false) => {
            //         #[cfg(not(feature = "v1"))]
            //         panic!("v1 feature is not enabled.");
            //
            //         #[cfg(feature = "v1")]
            //         rapidhash::v1::rapidhash_file(&mut file).expect("Failed to hash file.")
            //     }
            //     (false, true) => {
            //         #[cfg(not(any(feature = "v2", feature = "vlatest")))]
            //         panic!("v2 or vlatest feature is not enabled.");
            //
            //         #[cfg(feature = "v2")] {
            //             rapidhash::v2::rapidhash_file(&mut file).expect("Failed to hash file.")
            //         }
            //
            //         #[cfg(all(feature = "vlatest", not(feature = "v2")))] {
            //             rapidhash::rapidhash_file(&mut file).expect("Failed to hash file.")
            //         }
            //     }
            //     _ => unreachable!("Logic error."),
            // }
        }
    };

    println!("{hash}");
}
