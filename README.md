# One INI

The core implementation of an AST based, idiomatic INI parser which aims to provide an easy to implement and consistent INI-standard.

This reference implementation is provided as Rust-library and WASM-package.

<!-- markdownlint-disable -->

[![GitHub Actions](https://github.com/jedmao/editorconfig-ini/workflows/Rust/badge.svg?event=push)](https://github.com/jedmao/editorconfig-ini/actions)

<!-- markdownlint-restore -->

<!-- markdownlint-disable commands-show-output -->

The work on this project started with the search for an universal parser for the [EditorConfig INI file format specification](https://spec.editorconfig.org/#file-format).

## Development setup

Install Rust via rustup and Node.js 26 (including npm), then run:

```sh
cargo setup
```

This installs `wasm-pack` for WASM builds and `cargo-tarpaulin` for coverage.
Cargo uses the toolchain, components, and WASM target from `rust-toolchain.toml`.
The `--locked` install uses each tool's published lockfile; tool versions are not pinned.

## WASM

To use from [Web Assembly](https://webassembly.org/), compile with:

```sh
wasm-pack build --release --target nodejs
```

and run the (limited) WASM tests with:

```sh
wasm-pack test --node
```

You can call the generated JS wrapper with either:

```js
import { parse_to_json } from './pkg/one_ini.js'

const results = parse_to_json(`
root = true

[*]
# always use unix line endings
end_of_line = lf
`)

// {
//   "version": "0.1.0",
//   "body": [
//     { "type": "Pair", "key": "root", "value": "true" },
//     {
//       "type": "Section",
//       "name": "*",
//       "body": [
//         { "type": "Comment", "indicator": "#", "value": "always use unix line endings" },
//         { "type": "Pair", "key": "end_of_line", "value": "lf" }
//       ]
//     }
//   ]
// }
```

or:

```js
import { parse_to_uint32array, TokenTypes } from './pkg/editorconfig_ini.js'
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
//   TokenTypes.CommentValue, 20, 48,
//   TokenTypes.Key, 49, 60,
//   TokenTypes.Value, 63, 65
// ]
```

## Development

To get started with local development you will need some Rust tooling.

Rustup will install all language specific toolings for Rust, the cargo command installs the WASM-specific helpers.

```
rustup toolchain install stable
cargo setup
```


## WASM memory reuse check

Following a memory leak report (thanks @onmax), this minimal test checks memory
reuse on subsequent calls to `parse_to_json` and `parse_to_uint32array`.

Build a fresh release package and run the Node.js tests:

```sh
wasm-pack build --release --scope one-ini --target nodejs
node --test --test-reporter=tap tests/wasm-memory.cjs
```

Each parse export gets a fresh WASM instance and the same input containing 128
pairs with index-prefixed keys and values. After 100 warm-up calls, the test checks
that another 100 calls do not grow linear memory. Results are discarded. TAP output
includes the before/after memory sizes, and memory growth fails the test.

Continued growth flags a potential leak. Stable memory demonstrates reuse for
this workload, but cannot exclude every leak. Freed WASM allocations are reusable;
linear memory is not expected to shrink. This check measures WASM memory only.
