const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const dtsPath = path.join(__dirname, 'index.d.ts');
assert.ok(fs.existsSync(dtsPath), 'index.d.ts must exist for TypeScript users');

const dts = fs.readFileSync(dtsPath, 'utf8');

function hasDecl(pattern, label) {
  assert.match(dts, pattern, `Missing TypeScript declaration: ${label}`);
}

// Function type declarations should be present and typed.
hasDecl(
  /export declare function base64Encode\(input: string\): string/,
  'base64Encode(input: string): string'
);
hasDecl(
  /export declare function randomUserAgent\(\): string/,
  'randomUserAgent(): string'
);
hasDecl(
  /export declare function toSnakeCase\(input: string\): string/,
  'toSnakeCase(input: string): string'
);

// Builder should support chaining (this) and terminal build() string return type.
hasDecl(/export declare class TransformBuilder/, 'TransformBuilder class');
hasDecl(/constructor\(input: string\)/, 'TransformBuilder constructor');
hasDecl(/leetspeak\(\): this/, 'TransformBuilder.leetspeak(): this');
hasDecl(/urlEncode\(\): this/, 'TransformBuilder.urlEncode(): this');
hasDecl(/build\(\): string/, 'TransformBuilder.build(): string');

console.log('Type declaration tests passed.');
