#!/bin/sh

if ! [ -x "$(command -v wasm-pack)" ];
then
    cat <<- EOM
	Error: The required command "wasm-pack" was not found!
	       Try "cargo install wasm-pack" to install it.
EOM
    exit 1
fi

wasm-pack build --release --scope one-ini --target nodejs --locked

(cd ./pkg || exit; npm pkg set name='@one-ini/wasm' && npm pack --dry-run)
