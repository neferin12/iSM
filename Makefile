example_files = $(shell find example-files -type f)
c_source = $(shell find c/src -type f)
java_source = $(shell find java/app/src -type f)
ts_source = $(shell find ts/*/src -type f) $(shell find ts/*/package.json -type f)
python_source = $(shell find python/*.py -type f)

.PHONY: benchmark clean single_core_results multi_core_results

benchmark: benchmark.md

single_core_results: benchmarking/results/c_benchmark.csv benchmarking/results/java_benchmark.csv benchmarking/results/ts_benchmark.csv benchmarking/results/python_benchmark.csv

multi_core_results: benchmarking/results/c_multicore_benchmark.csv

python/venv:
	cd python && python3 -m venv venv

benchmark.md: benchmark.ipynb single_core_results multi_core_results
	jupyter nbconvert --to markdown --output $@ --no-input --execute $<

benchmarking/results/java_benchmark.csv: benchmarking/java_benchmark.sh $(java_source)
	cd benchmarking && bash java_benchmark.sh || rm $@
	
benchmarking/results/c_benchmark.csv: benchmarking/c_benchmark.sh $(c_source)
	cd benchmarking && bash c_benchmark.sh || rm $@
	
benchmarking/results/c_multicore_benchmark.csv: benchmarking/c_multicore_benchmark.sh $(c_source)
	cd benchmarking && bash c_multicore_benchmark.sh || rm $@
	
benchmarking/results/ts_benchmark.csv: benchmarking/ts_benchmark.sh $(ts_source)
	cd benchmarking && bash ts_benchmark.sh || rm $@

benchmarking/results/python_benchmark.csv: benchmarking/python_benchmark.sh $(python_source) python/venv
	cd benchmarking && bash python_benchmark.sh || rm $@

clean:
	rm -rf benchmarking/results/*_benchmark.csv
