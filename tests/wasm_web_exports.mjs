// Run after `wasm-pack build --release --target web --out-dir target/pkg-web --locked`.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import init, { parse_to_json, parse_to_uint32array, TokenTypes, version } from '../target/pkg-web/one_ini.js';

const wasm = await readFile(new URL('../target/pkg-web/one_ini_bg.wasm', import.meta.url));
const pkg = JSON.parse(await readFile(new URL('../target/pkg-web/package.json', import.meta.url)));
await init({ module_or_path: wasm });

assert.equal(version(), pkg.version);
assert.deepEqual(parse_to_json('name=🦀').body, [
  { type: 'Pair', key: 'name', value: '🦀' },
]);
const tokens = parse_to_uint32array(new TextEncoder().encode('name=🦀'));
assert.ok(tokens instanceof Uint32Array);
assert.deepEqual(Array.from(tokens), [TokenTypes.Key, 0, 4, TokenTypes.Value, 5, 9]);
assert.throws(() => parse_to_uint32array(Uint8Array.of(0xff)), Error);
