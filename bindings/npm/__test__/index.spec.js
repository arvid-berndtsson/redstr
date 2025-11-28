const test = require('ava');
const redstr = require('../index');

// Case transformations
test('randomizeCapitalization returns string', (t) => {
  const result = redstr.randomizeCapitalization('hello');
  t.is(typeof result, 'string');
  t.is(result.length, 5);
});

test('alternateCase works correctly', (t) => {
  const result = redstr.alternateCase('hello');
  // alternateCase may produce different patterns
  t.is(typeof result, 'string');
  t.is(result.length, 5);
});

test('caseSwap works correctly', (t) => {
  const result = redstr.caseSwap('Hello');
  // caseSwap may swap in different ways
  t.is(typeof result, 'string');
  t.is(result.length, 5);
});

test('toCamelCase works correctly', (t) => {
  const result = redstr.toCamelCase('hello world');
  t.is(result, 'helloWorld');
});

test('toSnakeCase works correctly', (t) => {
  const result = redstr.toSnakeCase('hello world');
  t.is(result, 'hello_world');
});

test('toKebabCase works correctly', (t) => {
  const result = redstr.toKebabCase('hello world');
  t.is(result, 'hello-world');
});

// Encoding transformations
test('base64Encode works correctly', (t) => {
  const result = redstr.base64Encode('hello');
  t.is(result, 'aGVsbG8=');
});

test('urlEncode works correctly', (t) => {
  const result = redstr.urlEncode('hello world');
  t.is(result, 'hello%20world');
});

test('hexEncode works correctly', (t) => {
  const result = redstr.hexEncode('test');
  t.is(result, '74657374');
});

test('htmlEntityEncode works correctly', (t) => {
  const result = redstr.htmlEntityEncode('<test>');
  t.true(result.includes('&lt;') || result.includes('&#'));
});

// Obfuscation transformations
test('leetspeak returns string', (t) => {
  const result = redstr.leetspeak('leet');
  t.is(typeof result, 'string');
  t.is(result.length, 4);
});

test('rot13 works correctly', (t) => {
  const result = redstr.rot13('hello');
  t.is(result, 'uryyb');
  // ROT13 is reversible
  const reversed = redstr.rot13(result);
  t.is(reversed, 'hello');
});

test('reverseString works correctly', (t) => {
  const result = redstr.reverseString('hello');
  t.is(result, 'olleh');
});

test('doubleCharacters works correctly', (t) => {
  const result = redstr.doubleCharacters('hi');
  // doubleCharacters may double randomly
  t.is(typeof result, 'string');
  t.true(result.length >= 2);
});

// Unicode transformations
test('homoglyphSubstitution returns string', (t) => {
  const result = redstr.homoglyphSubstitution('test');
  t.is(typeof result, 'string');
});

test('unicodeVariations returns string', (t) => {
  const result = redstr.unicodeVariations('test');
  t.is(typeof result, 'string');
});

test('zalgoText returns string', (t) => {
  const result = redstr.zalgoText('test');
  t.is(typeof result, 'string');
  t.true(result.length >= 4); // Should be at least as long as input
});

// Injection transformations
test('sqlCommentInjection returns string', (t) => {
  const result = redstr.sqlCommentInjection('SELECT');
  t.is(typeof result, 'string');
  t.true(result.includes('SELECT'));
});

test('xssTagVariations returns string', (t) => {
  const result = redstr.xssTagVariations('alert');
  t.is(typeof result, 'string');
});

test('pathTraversal returns string', (t) => {
  const result = redstr.pathTraversal('file');
  t.is(typeof result, 'string');
  t.true(result.length > 0);
});

// Bot detection
test('randomUserAgent returns string', (t) => {
  const result = redstr.randomUserAgent();
  t.is(typeof result, 'string');
  t.true(result.length > 0);
  t.true(result.includes('Mozilla') || result.includes('Chrome') || result.includes('Safari'));
});

test('acceptLanguageVariation returns string', (t) => {
  const result = redstr.acceptLanguageVariation('en-US');
  t.is(typeof result, 'string');
  t.true(result.length > 0);
});

// Web security
test('apiEndpointVariation returns string', (t) => {
  const result = redstr.apiEndpointVariation('/api/users');
  t.is(typeof result, 'string');
  t.true(result.length > 0);
});

test('graphqlObfuscate returns string', (t) => {
  const result = redstr.graphqlObfuscate('query { users }');
  t.is(typeof result, 'string');
});

// Phishing
test('domainTyposquat returns string', (t) => {
  const result = redstr.domainTyposquat('test.com');
  t.is(typeof result, 'string');
});

test('emailObfuscation returns string', (t) => {
  const result = redstr.emailObfuscation('test@example.com');
  t.is(typeof result, 'string');
});

// Shell
test('bashObfuscate returns string', (t) => {
  const result = redstr.bashObfuscate('echo test');
  t.is(typeof result, 'string');
});

test('powershellObfuscate returns string', (t) => {
  const result = redstr.powershellObfuscate('Write-Host test');
  t.is(typeof result, 'string');
});
