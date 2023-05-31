#!/bin/bash
cd ../c
mkdir build || true
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j
hyperfine \
--warmup 3 \
--parameter-scan num_threads 1 8 \
--export-csv ../../benchmarking/results/c_multicore_benchmark.csv \
'mpirun -np {num_threads} --use-hwthread-cpus ./cism ../../example-files/Wahlen.csv ../../example-files/Seminare.csv 1000000'