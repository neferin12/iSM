#!/bin/bash
for bench in $(find . -name "*_benchmark.sh"); do
    echo "Running $bench"
    ./$bench
done
jupyter nbconvert --to markdown --output benchmark.md --no-input --execute benchmark.ipynb