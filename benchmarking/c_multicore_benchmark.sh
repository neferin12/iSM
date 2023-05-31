#!/bin/bash
cd ../c
mkdir build || true
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j
hyperfine \
--warmup 3 'mpirun -np 2 ./cism ../../example-files/Wahlen.csv ../../example-files/Seminare.csv 10000'\
--parameter-scan num_threads 1 4 \
--export-csv ../../benchmarking/results/c_multicore_benchmark.csv \
'mpirun -np {cores} ./cism ../../example-files/Wahlen.csv ../../example-files/Seminare.csv 100000'
