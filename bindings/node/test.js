const assert = require('node:assert/strict');

const redstr = require('./index.js');

// Ensure core exports from the native binding are available.
assert.equal(typeof redstr.base64Encode, 'function');
assert.equal(typeof redstr.urlEncode, 'function');
assert.equal(typeof redstr.rot13, 'function');
assert.equal(typeof redstr.reverseString, 'function');
assert.equal(typeof redstr.TransformBuilder, 'function');

// Deterministic transformation checks.
assert.equal(redstr.base64Encode('hello'), 'aGVsbG8=');
assert.equal(redstr.urlEncode('hello world'), 'hello%20world');
assert.equal(redstr.hexEncode('AB'), '4142');
assert.equal(redstr.rot13('Hello'), 'Uryyb');
assert.equal(redstr.reverseString('abcd'), 'dcba');
assert.equal(redstr.toSnakeCase('HelloWorld'), 'hello_world');

// Builder API should chain and return expected output.
const chained = new redstr.TransformBuilder('hello')
  .base64()
  .urlEncode()
  .build();
assert.equal(chained, 'aGVsbG8%3D');

// Non-deterministic function smoke check.
const userAgent = redstr.randomUserAgent();
assert.equal(typeof userAgent, 'string');
assert.ok(userAgent.length > 0);

console.log('Node binding tests passed.');
