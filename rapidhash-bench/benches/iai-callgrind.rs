use std::hash::{BuildHasher, Hash, Hasher};
use std::hint::black_box;

use iai_callgrind::{library_benchmark, library_benchmark_group, Callgrind, CallgrindMetrics, LibraryBenchmarkConfig};

macro_rules! hash_benchmark {
    ($name:ident, $input:expr) => {
        #[library_benchmark]
        #[bench::rapidhash_q(&rapidhash::quality::RandomState::default(), $input)]
        #[bench::rapidhash_f(&rapidhash::fast::RandomState::default(), $input)]
        #[bench::foldhash_fast(&foldhash::fast::RandomState::default(), $input)]
        #[bench::foldhash_quality(&foldhash::quality::RandomState::default(), $input)]
        fn $name<H: BuildHasher + Sized, I: Hash>(hasher: &H, input: I) -> u64 {
            black_box(hasher.hash_one(black_box(input)))
        }
    };
}

macro_rules! hash_bytes {
    ($name:ident, $input:expr) => {
        #[library_benchmark]
        #[bench::rapidhash_q(&rapidhash::quality::RandomState::default(), $input)]
        #[bench::rapidhash_f(&rapidhash::fast::RandomState::default(), $input)]
        #[bench::foldhash_fast(&foldhash::fast::RandomState::default(), $input)]
        #[bench::foldhash_quality(&foldhash::quality::RandomState::default(), $input)]
        fn $name<H: BuildHasher + Sized>(hasher: &H, input: &[u8]) -> u64 {
            let mut hasher = hasher.build_hasher();
            hasher.write(black_box(input));
            black_box(hasher.finish())
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

hash_benchmark!(hash_string_uuid, "20ff9c15-3723-45b4-91d0-df234b4d852b");
hash_benchmark!(hash_string_date, "2025-05-18T15:11:10+0000");

hash_benchmark!(hash_bytes_32_one, b"abcdefg12345678abcdefg12345678");
hash_bytes!(hash_bytes_32_write, b"abcdefg12345678abcdefg12345678");

hash_benchmark!(hash_int_u8, 100u8);
hash_benchmark!(hash_int_u8_quad, (100u8, 10u8, 1u8, 200u8));
hash_benchmark!(hash_int_access_log, (
    0x5678_1248_1234_1245_5682_3452_1245_9857u128,
    0x1234_5678u32,
    chrono::NaiveDate::from_num_days_from_ce_opt(0x0123_4567i32).unwrap(),
    true
));
hash_benchmark!(hash_int_u32, 100u32);
hash_benchmark!(hash_int_u32_pair, (100u32, 10u32));
hash_benchmark!(hash_int_u64, 100u64);
hash_benchmark!(hash_int_u64_pair, (100u64, 10u64));

library_benchmark_group!(name = hash_string_group; benchmarks =
    hash_string_1,
    hash_string_8,
    hash_string_16,
    hash_string_32,
    hash_string_64,
    hash_string_uuid,
    hash_string_date,
);

library_benchmark_group!(name = hash_bytes_group; benchmarks =
    hash_bytes_32_one,
    hash_bytes_32_write,
);

library_benchmark_group!(name = hash_int_group; benchmarks =
    hash_int_u8,
    hash_int_u8_quad,
    hash_int_u32,
    hash_int_u32_pair,
    hash_int_u64,
    hash_int_u64_pair,
    hash_int_access_log,
);

iai_callgrind::main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default()
            .format([CallgrindMetrics::All])
        );
    library_benchmark_groups = hash_string_group,
    hash_bytes_group,
    hash_int_group,
);
