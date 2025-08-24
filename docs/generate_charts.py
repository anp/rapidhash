#!/usr/bin/python3
#
# /// script
# dependencies = [
#   "cbor2",
#   "matplotlib",
# ]
# ///
#
# To be run via: uv run docs/generate_charts.py

import glob
import sys

import cbor2
from matplotlib import pyplot as plt

def main():
    draw_hash()
#     draw_map()


def draw_hash():
    hash_settings = [
#         ("rapidhash_raw", "b"),
#         ("rapidhash_cc_rs", "orange"),
#         ("rapidhash_cc_v3", "b"),
#         ("rapidhash_cc_v2_1", "y"),
#         ("rapidhash_cc_v2", "g"),
#         ("rapidhash_cc_v1", "r"),
#         ("rapidhash_cc_v3na", "y"),
        ("gxhash", "m"),
        ("rapidhash-f", "b"),
        ("rapidhash-q", "b"),
        ("xxhash3_64", "0.8"),
        ("foldhash-f", "y"),
        ("foldhash-q", "y"),
        ("wyhash", "c"),
        ("metrohash", "0.8"),
        ("rustc-hash", "0.8"),
        ("ahash", "0.8"),
        ("xxhash64", "0.8"),
        ("t1ha", "0.8"),
        ("farmhash", "0.8"),
        ("seahash", "0.8"),
        ("highwayhash", "0.8"),
        ("fxhash", "r"),
        ("default", "k"),
    ]

    # Filter out gxhash if --portable is specified
    if "--portable" in sys.argv:
        hash_settings = [
            setting for setting in hash_settings
            if not setting[0].startswith("gxhash")
        ]

    if "--raw" in sys.argv:
        hash_settings = [
            ("rapidhash_raw", "b"),
            ("rapidhash_cc_rs", "c"),
            ("rapidhash_cc_v3", "y"),
            ("rapidhash_cc_v2_2", "g"),
#             ("rapidhash_cc_v2_1", "g"),
#             ("rapidhash_cc_v2", "g"),
            ("rapidhash_cc_v1", "r"),
        ]

    if "--small" in sys.argv:
        hash_settings = [
            ("rapidhash-f", "b"),
            ("foldhash-f", "y"),
#             ("fxhash", "r"),
        ]

    if "--small" in sys.argv and "--raw" in sys.argv:
        hash_settings = [
            ("rapidhash_cc_v3", "y"),
            ("rapidhash_cc_v3_1", "g"),
        ]

    hash_names = [hash_function for hash_function, _ in hash_settings]

    # also available: 65536, 524288000
    sizes = [2, 8, 16, 25, 50, 64, 80, 160, 256, 350, 1024, 4096, ]
    if "--small" in sys.argv:
        sizes = list(range(0, 300))

    latency_data = []
    throughput_data = []
    latency_data_u64 = []
    throughput_data_u64 = []
    latency_data_64k = []
    throughput_data_64k = []
    for (hash_function, _) in hash_settings:
        latency_row = []
        throughput_row = []

        prefix = "str"
        if "--small" in sys.argv:
            prefix = "small"

        for size in sizes:
            latency, throughput = load_latest_measurement_file("hash", hash_function, f"{prefix}_{size}")
            latency_row.append(latency)
            throughput_row.append(throughput)

        latency_data.append(latency_row)
        throughput_data.append(throughput_row)

        u64_measurement = "u64"
        s64k_measurement = "str_65536"
        if "--raw" in sys.argv:
            u64_measurement = "str_8"

        if "--small" in sys.argv:
            u64_measurement = "small_8"
            s64k_measurement = "small_256"

        latency, throughput = load_latest_measurement_file("hash", hash_function, u64_measurement)
        latency_data_u64.append(latency)
        throughput_data_u64.append(throughput)

        latency, throughput = load_latest_measurement_file("hash", hash_function, s64k_measurement)
        latency_data_64k.append(latency)
        throughput_data_64k.append(throughput)

    fig, axs = plt.subplots(2, 2, figsize=(12, 8), dpi=300)

    for i, (hash_function, color) in reversed(list(enumerate(hash_settings))):
        linestyle = "--" if hash_function.endswith("-f") else "-"
        linewidth = 1.0 if "--small" in sys.argv else 0.5

        axs[0, 0].plot(sizes, latency_data[i], label=hash_function, color=color, linestyle=linestyle, linewidth=linewidth)
        axs[0, 1].plot(sizes, throughput_data[i], label=hash_function, color=color, linestyle=linestyle, linewidth=linewidth)

        # Annotate the end of each line
        axs[0, 0].annotate(hash_function, (sizes[-1], latency_data[i][-1]), color=color,
                           xytext=(25, 0), textcoords='offset points', ha='left', va='center')
        axs[0, 1].annotate(hash_function, (sizes[-1], throughput_data[i][-1]), color=color,
                           xytext=(25, 0), textcoords='offset points', ha='left', va='center')

    for i, (hash_function, color) in enumerate(hash_settings):
        hatchstyle = "//" if hash_function.endswith("-f") else None
        edgecolor = "white" if hash_function.endswith("-f") else None
        print(hash_function, i, latency_data_u64[i], throughput_data_u64[i])
        # axs[1, 0].bar(hash_function, latency_data_u64[i], color=color, edgecolor=edgecolor, hatch=hatchstyle, zorder=3)
        axs[1, 0].bar(hash_function, throughput_data_u64[i], color=color, edgecolor=edgecolor, hatch=hatchstyle, zorder=3)
        axs[1, 1].bar(hash_function, throughput_data_64k[i], color=color, edgecolor=edgecolor, hatch=hatchstyle, zorder=3)

    labels = sizes
    if "--small" in sys.argv:
        labels = sizes[::20]

    axs[0, 0].set_title("Latency (byte stream)")
    axs[0, 0].set_xlabel("Input size (bytes)")
    axs[0, 0].set_ylabel("Latency (ns)")
    if "--small" not in sys.argv:
        axs[0, 0].set_xscale("log", base=2)
        axs[0, 0].set_yscale("log", base=10)
    axs[0, 0].set_xticks(labels)
    axs[0, 0].set_xticklabels(labels, rotation=90, ha="right")

    axs[0, 1].set_title("Throughput (byte stream)")
    axs[0, 1].set_xlabel("Input size (bytes)")
    axs[0, 1].set_ylabel("Throughput (GB/s)")
    if "--small" not in sys.argv:
        axs[0, 1].set_xscale("log", base=2)
        axs[0, 1].set_yscale("log", base=10)
    axs[0, 1].set_xticks(labels)
    axs[0, 1].set_xticklabels(labels, rotation=90, ha="right")

    if "--small" in sys.argv:
        axs[1, 0].set_title("Throughput (bytes, 8B)")
    else:
        axs[1, 0].set_title("Throughput (u64)")
    axs[1, 0].set_ylabel("Throughput (M Items/s)")
    axs[1, 0].set_xticks(range(len(hash_names)))
    axs[1, 0].set_xticklabels(hash_names, rotation=45, ha="right")
    axs[1, 0].grid(True, zorder=0, color="gainsboro")

    if "--small" in sys.argv:
        axs[1, 1].set_title("Throughput (bytes, 256B)")
    else:
        axs[1, 1].set_title("Throughput (bytes, 64kB)")
    axs[1, 1].set_ylabel("Throughput (GB/s)")
    axs[1, 1].set_xticks(range(len(hash_names)))
    axs[1, 1].set_xticklabels(hash_names, rotation=45, ha="right")
    axs[1, 1].grid(True, zorder=0, color="gainsboro")

    plt.tight_layout()
    plt.savefig("./docs/bench_hash.svg")

def draw_map():
    hash_settings = [
        ("rapidhash", "b"),
        ("default", "k"),
        ("fxhash", "r"),
        ("gxhash", "m"),
        ("wyhash", "c"),
        ("foldhash", "y"),
    ]

    hash_names = [hash_function.replace("_inline", "") for hash_function, _ in hash_settings]
    insert_benchmarks = [
        ("10000_emails", "emails"),
        ("450000_words", "words"),
        ("100000_u64", "u64"),
        ("10000_struct", "structs"),
    ]

    throughput_data = []
    for (hash_function, _) in hash_settings:
        throughput_row = []

        for (benchmark, _) in insert_benchmarks:
            _, throughput = load_latest_measurement_file("map", hash_function, benchmark)
            throughput_row.append(throughput)
        throughput_data.append(throughput_row)

    fig, axs = plt.subplots(2, 2, figsize=(12, 8), dpi=300)

    for i, (hash_function, color) in enumerate(hash_settings):
        axs[0, 0].bar(hash_function, throughput_data[i][0], color=color, zorder=3)
        axs[0, 1].bar(hash_function, throughput_data[i][1], color=color, zorder=3)
        axs[1, 0].bar(hash_function, throughput_data[i][2], color=color, zorder=3)
        axs[1, 1].bar(hash_function, throughput_data[i][3], color=color, zorder=3)

    for i, (_, benchmark) in enumerate(insert_benchmarks):
        x = int(i / 2)
        y = int(i % 2)
        assert 0 <= x <= 1
        assert 0 <= y <= 1
        print(i, x, y, benchmark)

        axs[x, y].set_title(f"Throughput ({benchmark})")
        axs[x, y].set_ylabel("Throughput (M Items/s)")
        axs[x, y].set_xticks(range(len(hash_names)))
        axs[x, y].set_xticklabels(hash_names, rotation=45, ha="right")
        axs[x, y].grid(True, zorder=0, color="gainsboro")

    plt.tight_layout()
    plt.savefig("bench_insert.svg")


def load_latest_measurement_file(group: str, hash_function: str, bench: str) -> (float, float):
    measurements = glob.glob(f"./target/criterion/data/main/{group}_{hash_function}/{bench}/measurement*")
    measurements.sort()
    assert len(measurements) > 0, f"No measurements found for {hash_function} {bench}"
    measurement_file = measurements[-1]

    with open(measurement_file, "rb") as f:
        data = cbor2.load(f)
        # print(data)
        latency = data["estimates"]["mean"]["point_estimate"]
        throughput_var = data["throughput"]

        if "Bytes" in throughput_var:
            size = throughput_var["Bytes"]
            throughput = ((1_000_000_000 / latency) * size) / 1_000_000_000
        else:
            size = throughput_var["Elements"] or 450000
            throughput = ((1_000_000_000 / latency) * size) / 1_000_000
        # print(hash_function, bench, size, latency, throughput)

    return latency, throughput


if __name__ == "__main__":
    main()
