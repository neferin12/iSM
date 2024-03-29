#!/bin/bash
cd ../c
mkdir build || true
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000,10000000 \
--export-csv ../../benchmarking/results/c_benchmark.csv \
'mpirun -np 1 ./cism ../../example-files/Wahlen.csv ../../example-files/Seminare.csv {runs}'
