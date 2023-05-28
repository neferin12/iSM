example_files = $(shell find example-files -type f)
c_source = $(shell find c/src -type f)
java_source = $(shell find java/app/src -type f)
ts_source = $(shell find ts/*/src -type f) $(shell find ts/*/package.json -type f)

.PHONY: benchmark clean

benchmark: benchmark.md

benchmark.md: benchmark.ipynb benchmarking/results/c_benchmark.csv benchmarking/results/java_benchmark.csv benchmarking/results/ts_benchmark.csv
	jupyter nbconvert --to markdown --output $@ --no-input --execute $<

benchmarking/results/java_benchmark.csv: benchmarking/java_benchmark.sh $(java_source)
	cd benchmarking && bash java_benchmark.sh || rm $@
	
benchmarking/results/c_benchmark.csv: benchmarking/c_benchmark.sh $(c_source)
	cd benchmarking && bash c_benchmark.sh || rm $@
	
benchmarking/results/ts_benchmark.csv: benchmarking/ts_benchmark.sh $(ts_source)
	cd benchmarking && bash ts_benchmark.sh || rm $@

clean:
	rm -rf benchmarking/results/*_benchmark.csv
