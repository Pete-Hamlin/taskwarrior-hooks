all: install hooks

install: 
    cargo build --release
    cp target/release/filter_count ~/.local/bin

hooks:
    cp hooks/* ~/.task/hooks/

build:
    cargo build
