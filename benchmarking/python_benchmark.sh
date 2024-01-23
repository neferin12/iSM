#!/bin/bash
cd ../python
pip install -r requirements.txt
hyperfine \
--warmup 3 \
--parameter-list runs 100,1000,10000,100000,1000000,10000000 \
--export-csv ../benchmarking/results/python_benchmark.csv \
'./main.py ../example-files/Wahlen.csv ../example-files/Seminare.csv {runs}'
