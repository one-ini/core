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

node -e 'n="./pkg/package.json";p=require(n);p.name="@one-ini/wasm";fs.writeFileSync(n,JSON.stringify(p,null,2))'
(cd ./pkg || exit; npm pack --dry-run)
