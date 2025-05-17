use std::hash::{BuildHasher, Hash};
use std::hint::black_box;

use iai_callgrind::{library_benchmark, library_benchmark_group, main};

macro_rules! hash_benchmark {
    ($name:ident, $input:expr) => {
        #[library_benchmark]
        #[bench::rapidhash_v1(&rapidhash::v1::RapidBuildHasher::default(), $input)]
        #[bench::rapidhash_v2(&rapidhash::v2::RapidBuildHasher::default(), $input)]
        #[bench::foldhash_quality(&foldhash::quality::FixedState::default(), $input)]
        #[bench::foldhash_fast(&foldhash::fast::FixedState::default(), $input)]
        fn $name<H: BuildHasher + Sized, I: Hash>(hasher: &H, input: I) -> u64 {
            black_box(hasher.hash_one(black_box(input)))
        }
    };
}

hash_benchmark!(hash_string_1, "a");
hash_benchmark!(hash_string_8, "abcdefg");
hash_benchmark!(hash_string_16, "abcdefg12345678");
hash_benchmark!(hash_string_32, "abcdefg12345678abcdefg12345678");
hash_benchmark!(
    hash_string_64,
    "abcdefg12345678abcdefg12345678abcdefg12345678abcdefg12345678"
);

library_benchmark_group!(name = hash_string_group; benchmarks =
    hash_string_1,
    hash_string_8,
    hash_string_16,
    hash_string_32,
    hash_string_64
);
main!(library_benchmark_groups = hash_string_group);
