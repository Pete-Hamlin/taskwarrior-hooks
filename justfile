all: install hooks

install: 
    cargo build --release
    cp target/release/hook_* ~/.local/bin

hooks:
    cp hooks/* ~/.task/hooks/

build:
    cargo build
