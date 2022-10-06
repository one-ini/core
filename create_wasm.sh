#!/bin/sh

if ! [ -x "$(command -v wasm-pack)" ];
then
    cat <<- EOM
	Error: The required command "wasm-pack" was not found!
	       Try "cargo install wasm-pack" to install it.
EOM
    exit 1
fi

cargo clean
wasm-pack build --release --scope one-ini --target nodejs
