#!/bin/bash
cd ../java
./gradlew distZip
cd app/build/distributions
rm -rf app || true
unzip app.zip
cd app/bin
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000,10000000 \
--export-csv ../../../../../../benchmarking/results/java_benchmark.csv \
'./app ../../../../../../example-files/Wahlen.csv ../../../../../../example-files/Seminare.csv {runs}'
