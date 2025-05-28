use std::fs::File;
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
/// On rapidhash V1 and V2:
/// This will first check the metadata of the file to get the length, and then stream the file.
///
/// On rapidhash V3:
/// This will stream the file using an 8KB buffer without any metadata checks.
///
/// ```bash
/// rapidhash example.txt
/// 8543579700415218186
/// ```
///
/// ## Reading stdin
///
/// On rapidhash V1 and V2:
/// Because of how rapidhash is seeded using the data length, the length must be known at the start
/// of the stream. Therefore, reading from stdin is not recommended as it will cache the entire
/// input in memory before being able to hash it.
///
/// On rapidhash V3:
/// This will stream from stdin using an 8KB buffer.
///
/// ```shell
/// echo "example" | rapidhash
/// 8543579700415218186
/// ```
pub fn main() {
    #[cfg(not(feature = "std"))] {
        panic!("CLI must be compiled with the `std` feature. Try: `cargo install rapidhash --all-features`");
    }

    let args: Vec<String> = std::env::args().collect();
    // TODO: add support for `--version`, multiple output types (hex, decimal).
    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 2 {
        println!("Usage: rapidhash <version> [opts] [filename]");
        println!("<version>");
        println!("  --v1    Use v1 hashing algorithm (no streaming)");
        println!("  --v2    Use v2 hashing algorithm (no streaming)");
        println!("  --v2.1  Use v2.1 hashing algorithm (no streaming)");
        println!("  --v2.2  Use v2.2 hashing algorithm (no streaming)");
        println!("  --v3    Use v3 hashing algorithm");
        println!("  --rs    Use the rust algorithm variant (non-standard)");
        println!("[opts]");
        println!("  --protected  Use the protected variant (default: false)");
        println!("[filename]");
        println!("  Providing a filename is optional and will read a file directly. Otherwise input");
        println!("  is read from stdin.");
        println!();
        println!("Note that only some rapidhash versions support streaming, others require");
        println!("buffering the entire input in memory.");
        println!();
        println!("Docs: https://github.com/hoxxep/rapidhash?tab=readme-ov-file#cli");
        return;
    }

    // file name is the first non-option argument
    let filename = args.iter().skip(1).filter(|a| !a.starts_with('-')).next();

    // get the rapidhash version from the command line arguments
    let version = RapidhashVersion::new(&args[1..])
        .expect("You must specify a single rapidhash version to use. See --help for more.");

    let hash: u64 = match filename {
        None => {
            version.hash_stdin()
        }

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        Some(filename) => {
            let mut file = File::open(filename).expect("Could not open file.");
            version.hash_file(&mut file)
        }
    };

    println!("{hash}");
}

/// Ohhhh boy, this one ain't pretty. Sorry!
enum RapidhashVersion {
    V1 { protected: bool },
    V2 { protected: bool, version: u8 },
    V3 { protected: bool },
    Rs { protected: bool },
}

impl RapidhashVersion {
    pub fn new(args: &[String]) -> Option<Self> {
        let v1 = args.iter().any(|a| a == "--v1");
        let v2 = args.iter().any(|a| a == "--v2");
        let v2_1 = args.iter().any(|a| a == "--v2.1");
        let v2_2 = args.iter().any(|a| a == "--v2.2");
        let v3 = args.iter().any(|a| a == "--v3");
        let rs = args.iter().any(|a| a == "--rs");
        let protected = args.iter().any(|a| a == "--protected");

        if 1 != (v1 as u8) + (v2 as u8) + (v2_1 as u8) + (v2_2 as u8) + (v3 as u8) + (rs as u8) {
            return None;
        }

        if v1 {
            Some(RapidhashVersion::V1 { protected })
        } else if v2 {
            Some(RapidhashVersion::V2 { protected, version: 0 })
        } else if v2_1 {
            Some(RapidhashVersion::V2 { protected, version: 1 })
        } else if v2_2 {
            Some(RapidhashVersion::V2 { protected, version: 2 })
        } else if v3 {
            Some(RapidhashVersion::V3 { protected })
        } else if rs {
            Some(RapidhashVersion::Rs { protected })
        } else {
            None
        }
    }

    pub fn hash_stdin(&self) -> u64 {
        match self {
            RapidhashVersion::V1 { protected } => {
                let mut buffer = Vec::with_capacity(1024);
                std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");

                if *protected {
                    rapidhash::v1::rapidhash_v1_inline::<false, true>(&buffer, rapidhash::v1::RAPID_SEED)
                } else {
                    rapidhash::v1::rapidhash_v1_inline::<false, false>(&buffer, rapidhash::v1::RAPID_SEED)
                }
            },
            RapidhashVersion::V2 { protected, version } => {
                let mut buffer = Vec::with_capacity(1024);
                std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");

                match version {
                    0 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_inline::<0, false, true>(&buffer, rapidhash::v2::RAPID_SEED)
                        } else {
                            rapidhash::v2::rapidhash_v2_inline::<0, false, false>(&buffer, rapidhash::v2::RAPID_SEED)
                        }
                    }
                    1 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_inline::<1, false, true>(&buffer, rapidhash::v2::RAPID_SEED)
                        } else {
                            rapidhash::v2::rapidhash_v2_inline::<1, false, false>(&buffer, rapidhash::v2::RAPID_SEED)
                        }
                    }
                    2 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_inline::<2, false, true>(&buffer, rapidhash::v2::RAPID_SEED)
                        } else {
                            rapidhash::v2::rapidhash_v2_inline::<2, false, false>(&buffer, rapidhash::v2::RAPID_SEED)
                        }
                    }
                    _ => {
                        panic!("Unsupported v2 version: {version}. Supported versions are 0, 1, and 2.");
                    }
                }
            },
            RapidhashVersion::V3 { protected } => {
                if *protected {
                    rapidhash::v3::rapidhash_v3_file_inline::<_, true>(std::io::stdin(), rapidhash::v3::RAPID_SEED)
                        .expect("Could not read from stdin.")
                } else {
                    rapidhash::v3::rapidhash_v3_file_inline::<_, false>(std::io::stdin(), rapidhash::v3::RAPID_SEED)
                        .expect("Could not read from stdin.")
                }
            },
            RapidhashVersion::Rs { protected } => {
                if *protected {
                    rapidhash::inner::rapidhash_rs_file_inline::<_, true>(std::io::stdin(), rapidhash::inner::RAPID_SEED)
                        .expect("Could not read from stdin.")
                } else {
                    rapidhash::inner::rapidhash_rs_file_inline::<_, false>(std::io::stdin(), rapidhash::inner::RAPID_SEED)
                        .expect("Could not read from stdin.")
                }
            }
        }
    }

    pub fn hash_file(&self, reader: &mut File) -> u64 {
        match self {
            RapidhashVersion::V1 { protected } => {
                if *protected {
                    rapidhash::v1::rapidhash_v1_file_inline::<true>(reader, rapidhash::v1::RAPID_SEED)
                        .expect("Failed to hash file.")
                } else {
                    rapidhash::v1::rapidhash_v1_file_inline::<false>(reader, rapidhash::v1::RAPID_SEED)
                        .expect("Failed to hash file.")
                }
            },
            RapidhashVersion::V2 { protected, version } => {
                match version {
                    0 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_file_inline::<0, true>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        } else {
                            rapidhash::v2::rapidhash_v2_file_inline::<0, false>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        }
                    }
                    1 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_file_inline::<1, true>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        } else {
                            rapidhash::v2::rapidhash_v2_file_inline::<1, false>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        }
                    }
                    2 => {
                        if *protected {
                            rapidhash::v2::rapidhash_v2_file_inline::<2, true>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        } else {
                            rapidhash::v2::rapidhash_v2_file_inline::<2, false>(reader, rapidhash::v2::RAPID_SEED)
                                .expect("Failed to hash file.")
                        }
                    }
                    _ => {
                        panic!("Unsupported v2 version: {version}. Supported versions are 0, 1, and 2.");
                    }
                }
            },
            RapidhashVersion::V3 { protected} => {
                if *protected {
                    rapidhash::v3::rapidhash_v3_file_inline::<_, true>(reader, rapidhash::v3::RAPID_SEED)
                        .expect("Failed to hash file.")
                } else {
                    rapidhash::v3::rapidhash_v3_file_inline::<_, false>(reader, rapidhash::v3::RAPID_SEED)
                        .expect("Failed to hash file.")
                }
            },
            RapidhashVersion::Rs { protected} => {
                if *protected {
                    rapidhash::inner::rapidhash_rs_file_inline::<_, true>(reader, rapidhash::inner::RAPID_SEED)
                        .expect("Failed to hash file.")
                } else {
                    rapidhash::inner::rapidhash_rs_file_inline::<_, false>(reader, rapidhash::inner::RAPID_SEED)
                        .expect("Failed to hash file.")
                }
            },
        }
    }
}
