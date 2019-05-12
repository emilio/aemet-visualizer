CSVS := $(wildcard data/**/*.csv)
RUST_CODE := $(wildcard src/*.rs)

app/static/data: $(CSVS) $(RUST_CODE) Cargo.toml Cargo.lock
	mkdir -p app/static/data
	cargo run --release -- app/static/data
