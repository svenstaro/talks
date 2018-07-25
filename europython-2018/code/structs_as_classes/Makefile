rust_structs.so: src/
	cargo build --release
	cp -a target/release/librust_structs.so rust_structs.so

.PHONY: run
run: rust_structs.so
	python run.py

.PHONY: clean
clean:
	rm -rf target rust_structs.so
