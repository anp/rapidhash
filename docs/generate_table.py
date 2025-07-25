# Borrowed from foldhash to generate benchmarking tables.

import csv
import json
import cbor2
import sys
import polars as pl

from pathlib import Path

base_path = Path("../target/criterion/")
csv_path = Path("bench.csv")

def extract():
#     if csv_path.exists():
#         print(f"File {csv_path} already exists. Remove it to re-cache results.")
#         return

    if not base_path.exists():
        print(f"Path {base_path} does not exist. Run the benchmarks first.")
        sys.exit(1)

    with csv_path.open("w") as csv_file:
        csv_file.write("bench,distr,hash,ns\n")

        for path in base_path.glob("data/main/realworld*/*/benchmark.cbor"):
            benchmark_path = path
            sample_path = sorted(path.parent.glob("measurement*"))[-1]

            with benchmark_path.open("rb") as benchmark_file, sample_path.open("rb") as sample_file:
                name = cbor2.load(benchmark_file)["id"]["function_id"]
                samples = cbor2.load(sample_file)

#                 sample_times = sorted([t / n for t, n in zip(samples["times"], samples["iters"])])
#                 robust = sample_times[len(sample_times) // 10]

                robust = samples["estimates"]["mean"]["point_estimate"]
                csv_file.write(",".join(name.split("-", 2)) + "," + str(robust) + "\n")

def table():
    MAP_SIZE = 1000
    SET_BUILD_FACTOR = 10 * MAP_SIZE

    distr_order = [
        "u32",
        "u32pair",
        "u64",
        "u64lobits",
        "u64hibits",
        "u64pair",
        "ipv4",
        "ipv6",
        "rgba",
        "strenglishword",
        "struuid",
        "strurl",
        "strdate",
        "accesslog",
        "kilobyte",
        "tenkilobyte",
    ]

    name_repl = {
        "rapidhash-f": "rapidhash-f",
        "rapidhash-q": "rapidhash-q",
        "foldhash-fast": "foldhash-f",
        "foldhash-quality": "foldhash-q",
    }

    bench_order = ["hashonly", "lookupmiss", "lookuphit", "setbuild"]
    hash_order = [
#         "rapidhash-f",
        "rapidhash-q",
#         "foldhash-f",
        "foldhash-q",
#         "gxhash",
#         "fxhash",
#         "ahash",
#         "siphash"
    ]

    distr_order_df = pl.DataFrame({"distr": distr_order, "distr_order_idx": range(len(distr_order))})
    bench_order_df = pl.DataFrame({"bench": bench_order, "bench_order_idx": range(len(bench_order))})
    hash_order_df = pl.DataFrame({"hash": hash_order, "hash_order_idx": range(len(hash_order))})

    df = (
        pl.scan_csv(csv_path)
            .with_columns(pl.col.hash.replace(name_repl))
            .with_columns(ns = pl.col.ns / pl.when(pl.col.bench == "setbuild").then(SET_BUILD_FACTOR).otherwise(1))
            .join(distr_order_df.lazy(), on="distr")
            .join(bench_order_df.lazy(), on="bench")
            .join(hash_order_df.lazy(), on="hash")
            .sort(["distr_order_idx", "distr", "bench_order_idx", "hash_order_idx"])
            .select(pl.col.distr, pl.col.bench, pl.col.hash, pl.col.ns)
            .collect()
    )

    with pl.Config(tbl_rows=-1, float_precision=2, tbl_cell_alignment="RIGHT"):
        print(df.pivot("hash", values="ns"))
        print(
            df
                .with_columns(rank = pl.col.ns.rank().over("distr", "bench"))
                .group_by("hash", maintain_order=True)
                .agg(
                    avg_rank = pl.col.rank.mean(),
                    geometric_mean = pl.col.ns.log().mean().exp()
                )
                .transpose(include_header=True, header_name="metric", column_names="hash")
        )

def main():
    extract()
    table()

if __name__ == "__main__":
    main()

