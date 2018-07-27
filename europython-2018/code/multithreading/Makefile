multithreading.so: src/
	cargo build --release
	cp -a target/release/libmultithreading.so multithreading.so

.PHONY: run
run: multithreading.so
	time python run.py

.PHONY: clean
clean:
	rm -rf target multithreading.so
