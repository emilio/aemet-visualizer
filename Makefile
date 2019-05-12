CSVS := $(wildcard data/**/*.csv)
RUST_CODE := $(wildcard src/*.rs)

app/static/data.json: $(CSVS) $(RUST_CODE) Cargo.toml Cargo.lock
	mkdir -p app/static
	cargo run --release > $@
