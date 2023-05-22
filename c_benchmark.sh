#!/bin/bash
cd c
mkdir build || true
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000 \
--export-csv ../../c_benchmark.csv \
'./cism ../../example-files/Wahlen.csv ../../example-files/Seminare.csv {runs}'