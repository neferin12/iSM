#!/bin/bash
cd ../rust
cargo build --release
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000,10000000 \
--export-csv ../benchmarking/results/rust_benchmark.csv \
'./target/release/rism --students-path ../example-files/Wahlen.csv --seminars-path ../example-files/Seminare.csv --iterations {runs}'
