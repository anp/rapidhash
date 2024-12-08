use std::io::{BufReader, Read};
use rapidhash::rapidhash_file;

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

    let hash = match hash_arg {
        None => {
            let mut buffer = Vec::with_capacity(1024);
            std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");
            rapidhash::rapidhash(&buffer)
        }
        Some(filename) => {
            if filename == "--help" {
                println!("Usage: rapidhash [filename]");
                return;
            }

            let mut file = std::fs::File::open(filename).expect("Could not open file.");
            rapidhash_file(&mut file).expect("Failed to hash file.")
        }
    };

    println!("{hash}");
}

/// This is some of the least-idiomatic rust I've ever written and is a butchering between
/// `std::io::Read` and `std::iter::Iterator`. This is a hack to get the `rapidhash_stream` to play
/// nice because it needs to know the length of the input stream.
struct ExactSizeBufReader {
    reader: BufReader<std::fs::File>,
    position: usize,
    total_size: usize,
    status: std::io::Result<()>,
}

impl ExactSizeBufReader {
    fn new(file: std::fs::File) -> std::io::Result<Self> {
        let total_size = file.metadata()?.len() as usize;
        Ok(Self {
            reader: BufReader::new(file),
            position: 0,
            total_size,
            status: Ok(()),
        })
    }
}

impl Iterator for ExactSizeBufReader {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.total_size {
            // TODO: check metadata of file hasn't changed underneath us (eg. appends)
            return None;
        }

        let mut buffer = [0; 1];
        if let Err(e) = self.reader.read_exact(&mut buffer) {
            self.status = Err(e);
            return None;
        };

        self.position += 1;
        Some(buffer[0])
    }
}

impl ExactSizeIterator for ExactSizeBufReader {
    #[inline]
    fn len(&self) -> usize {
        self.total_size - self.position
    }
}
