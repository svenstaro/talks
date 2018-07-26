simple_example.so: src/
	cargo build --release
	cp -a target/release/libsimple_example.so simple_example.so

.PHONY: run_rust
run_rust: simple_example.so
	time python with_rust.py

.PHONY: run_cpython
run_cpython:
	time python python_only.py

.PHONY: run_pypy
run_pypy:
	time pypy3 python_only.py

.PHONY: run_cython
run_cython:
	python setup.py build_ext --inplace
	time python -c "import with_cython"

.PHONY: clean
clean:
	rm -rf target simple_example.so
