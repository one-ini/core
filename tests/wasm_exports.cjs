// Run after `wasm-pack build --release --target nodejs --locked`.
const assert = require('node:assert/strict');
const { parse_to_json, parse_to_uint32array, TokenTypes, version } = require('../pkg/one_ini.js');

assert.equal(version(), require('../pkg/package.json').version);
assert.deepEqual(parse_to_json('name=🦀').body, [
  { type: 'Pair', key: 'name', value: '🦀' },
]);

const tokens = parse_to_uint32array(new TextEncoder().encode('name=🦀'));
assert.ok(tokens instanceof Uint32Array);
assert.deepEqual(Array.from(tokens), [TokenTypes.Key, 0, 4, TokenTypes.Value, 5, 9]);
assert.throws(() => parse_to_uint32array(Uint8Array.of(0xff)), Error);
