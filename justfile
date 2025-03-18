install: build
    cp target/release/filter_count ~/.local/bin

build:
    cargo build
