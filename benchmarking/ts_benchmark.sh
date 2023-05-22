#!/bin/bash
cd ../ts/cli
yarn run build
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000 \
--export-csv ../../benchmarking/results/ts_benchmark.csv \
'./bin/run run ../../example-files/Wahlen.csv ../../example-files/Seminare.csv {runs}'