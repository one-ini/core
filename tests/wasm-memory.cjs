const assert = require('node:assert/strict');
const { test } = require('node:test');

// Use distinct keys and values to exercise parsing many different entries.
// Reuse this input across calls so input creation cannot affect the comparison.
const text = '[*]\n' + Array.from(
  { length: 128 },
  (_, index) => `# sample\n${index}_key=${index}_value\n`,
).join('');

for (const [name, input] of [
  ['parse_to_json', text],
  ['parse_to_uint32array', Buffer.from(text)],
]) {
  test(`${name} reuses WASM memory on subsequent calls`, (t) => {
    // The generated wrapper keeps its WASM instance private. Temporarily capture
    // its memory during construction, then restore the original constructor.
    const OriginalInstance = WebAssembly.Instance;
    let memory;
    let api;
    try {
      WebAssembly.Instance = class extends OriginalInstance {
        constructor(...args) {
          super(...args);
          memory = this.exports.memory;
        }
      };
      // Give each export a fresh instance so previous growth cannot mask a leak.
      delete require.cache[require.resolve('../pkg/one_ini.js')];
      api = require('../pkg/one_ini.js');
    } finally {
      WebAssembly.Instance = OriginalInstance;
    }
    assert.ok(memory instanceof WebAssembly.Memory);

    // Warm up the allocator before measuring: initial memory growth is expected.
    // Discard each result so subsequent calls can reuse freed WASM allocations.
    for (let i = 0; i < 100; i++) api[name](input);
    const before = memory.buffer.byteLength;
    for (let i = 0; i < 100; i++) api[name](input);
    const after = memory.buffer.byteLength;

    // Linear memory does not shrink when allocations are freed. For this fixed
    // workload it should plateau; further growth flags a potential memory leak.
    t.diagnostic(`WASM memory: ${before} -> ${after} bytes`);
    assert.equal(after, before, 'WASM memory grew after warm-up');
  });
}
