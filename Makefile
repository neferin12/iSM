example_files = $(shell find example-files)
c_source = $(shell find c/src)
java_source = $(shell find java/app/src)
ts_source = $(shell find ts/src)

.PHONY: benchmark

benchmark: benchmark.md

benchmark.md: benchmark.ipynb benchmarking/results/c_benchmark.csv benchmarking/results/java_benchmark.csv benchmarking/results/ts_benchmark.csv
	jupyter nbconvert --to markdown --output $@ --no-input --execute $<

benchmarking/results/%_benchmark.csv: benchmarking/%_benchmark.sh $(%_source)
	cd benchmarking && bash $*_benchmark.sh || rm $@
