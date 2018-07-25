europython.so: src/
	cargo build --release
	cp -a target/release/libsimple_bind.so europython.so

.PHONY: run_rust
run: europython.so
	python run.py

.PHONY: clean
clean:
	rm -rf target europython.so
