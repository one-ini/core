# One INI

The core implementation of an AST based, idiomatic INI parser which aims to provide an easy to implement and consistent INI-standard.

This reference implementation is provided as Rust-library and WASM-package.

<!-- markdownlint-disable -->

[![GitHub Actions](https://github.com/one-ini/core/actions/workflows/rust.yml/badge.svg)](https://github.com/one-ini/core/actions/workflows/rust.yml)

<!-- markdownlint-restore -->

<!-- markdownlint-disable commands-show-output -->

The work on this project started with the search for an universal parser for the [EditorConfig INI file format specification](https://spec.editorconfig.org/#file-format).

## Development

Use stable Rust with `rustfmt` and `clippy`. The crate uses edition 2024 and
supports Rust 1.85 or newer; CI checks that minimum version as well as stable
Rust on Linux, macOS and Windows. The parser requires `std`.

```sh
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
```

`Cargo.lock` is checked in to make development and CI reproducible. Update
dependencies deliberately with `cargo update`, then run the native and Wasm
checks. Downstream Rust applications resolve their own dependency versions.

For each parser change:

1. Add a regression test for the desired behavior and run it alone, for example
   `cargo test --locked --test parser_contract unicode_offsets`.
2. Confirm that it fails for the expected reason, then implement the change.
3. Refactor with the tests green, then run the complete test and lint commands.
4. For binding changes, also run the Node.js and browser suites below.

The existing integration tests cover normalization and token extraction.
`tests/parser_contract.rs` covers byte offsets, ordering and tolerant parsing;
`tests/ast_roundtrip.rs` covers AST serialization. Both Wasm runtimes share the
binding contracts in `tests/common/wasm.rs`. Doctests exercise the Rust API.
After building the Node package, `node tests/wasm_exports.cjs` checks the
generated JavaScript exports, typed arrays and thrown errors. After building
the web package in `target/pkg-web`, `node tests/wasm_web_exports.mjs` checks
its initialization and exports using Node.js; the browser suite also runs in CI.

The Rust API (`parse`, `parse_to_vec`, AST types) has no JavaScript dependencies
on native targets. Wasm bindings live in `src/wasm.rs` and are built for Wasm
targets only. The default allocator is used; a panic hook is available with
`--features console_error_panic_hook` and is initialized by the bindings.

Parsing currently accepts a valid prefix and may ignore an incomplete or invalid
suffix. Strict input validation is a future API decision. Displaying an AST
normalizes whitespace and line endings; it does not preserve the original
formatting. Token offsets refer to UTF-8 bytes with an exclusive end offset.

## WASM

Install the JavaScript Wasm target and build tooling once (Node.js is needed
for Node tests; Chrome and ChromeDriver are needed for browser tests):

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
```

To use from [Web Assembly](https://webassembly.org/), compile with:

```sh
wasm-pack build --release --target nodejs --locked
# For browsers / bundlers using ES modules instead:
wasm-pack build --release --target web --out-dir target/pkg-web --locked
```

Run the Wasm tests in both runtimes (native `cargo test` does not execute them):

```sh
wasm-pack test --node --locked --all-features --test node
wasm-pack test --headless --chrome --locked --all-features --test web
```

The exports remain `version`, `TokenTypes`, `parse_to_json` and
`parse_to_uint32array`. Despite its historical name, `parse_to_json` returns a
JavaScript object, not a JSON string. Binding failures throw JavaScript errors;
the Rust binding function now returns `Result<JsValue, JsError>`.

You can call the generated Node.js wrapper with either:

```js
import { parse_to_json } from './pkg/one_ini.js'

const results = parse_to_json(`
root = true

[*]
# always use unix line endings
end_of_line = lf
`)

// {
//   "version": "0.2.2",
//   "body": [
//     { "type": "Pair", "key": "root", "value": "true" },
//     {
//       "type": "Section",
//       "name": "*",
//       "body": [
//         { "type": "Comment", "indicator": "#", "value": " always use unix line endings" },
//         { "type": "Pair", "key": "end_of_line", "value": "lf" }
//       ]
//     }
//   ]
// }
```

or:

```js
import { parse_to_uint32array, TokenTypes } from './pkg/one_ini.js'
const buf = Buffer.from(`
root = true

[*]
# always use unix line endings
end_of_line = lf
`, 'utf8')
const ary = parse_to_uint32array(buf)

// Array with token type, start byte offset, end byte offset for each token
// Uint32Array(21) [
//   TokenTypes.Key, 1, 5,
//   TokenTypes.Value, 8, 12,
//   TokenTypes.Section, 15, 16,
//   TokenTypes.CommentIndicator, 18, 19,
//   TokenTypes.CommentValue, 19, 48,
//   TokenTypes.Key, 49, 60,
//   TokenTypes.Value, 63, 65
// ]
```

## Releases

Publish a GitHub release from a tag matching `Cargo.toml`, either `0.2.2` or
`v0.2.2` for the current version. The release workflow rejects mismatched tags,
runs the complete native and Wasm CI against the release commit, then builds
and checks optimized Node.js and web packages.

It attaches three files to the existing GitHub release:

- `one-ini-0.2.2-nodejs.tgz`: CommonJS package for Node.js.
- `one-ini-0.2.2-web.tgz`: ES module package for browsers, including Wasm initialization.
- `SHA256SUMS`: SHA-256 checksums for both archives.

Each archive contains the package metadata, JavaScript bindings, TypeScript
declarations, Wasm binary, README and license under `package/`. Both packages
use the name `@one-ini/wasm`; choose the variant for your runtime. For example,
after downloading the Node.js archive:

```sh
sha256sum --ignore-missing -c SHA256SUMS
npm install ./one-ini-0.2.2-nodejs.tgz
```

For the web package, serve the extracted files and initialize the module before
calling the parser:

```js
import init, { parse_to_json } from './package/one_ini.js';

await init();
const ast = parse_to_json('root=true');
```

The workflow uses the built-in `GITHUB_TOKEN` with `contents: write` for
[release uploads](https://cli.github.com/manual/gh_release_upload). No npm
registry setup or additional publishing token is required. Configure any
required approvals in the GitHub environment `publish`. Existing release files
are not overwritten; a rerun fails if an asset with the same name already exists.

The workflow can also be called from another workflow on a version tag, provided
that the GitHub release already exists and the caller grants `contents: write`.
