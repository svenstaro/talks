simple_example.so: src/
	cargo build --release
	cp -a target/release/libsimple_example.so simple_example.so

.PHONY: run_rust
run_rust: simple_example.so
	hyperfine 'python with_rust.py'

.PHONY: run_native
run_python:
	hyperfine 'python python_only.py'

.PHONY: clean
clean:
	rm -rf target simple_example.so
