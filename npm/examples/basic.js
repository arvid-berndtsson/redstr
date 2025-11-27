// Basic usage examples for @redstr/core

const redstr = require('../index');

console.log('=== redstr Node.js Bindings - Basic Examples ===\n');

// Case transformations
console.log('Case Transformations:');
console.log('randomizeCapitalization:', redstr.randomizeCapitalization('Hello World'));
console.log('alternateCase:', redstr.alternateCase('Hello World'));
console.log('caseSwap:', redstr.caseSwap('Hello World'));
console.log('toCamelCase:', redstr.toCamelCase('hello world'));
console.log('toSnakeCase:', redstr.toSnakeCase('Hello World'));
console.log('toKebabCase:', redstr.toKebabCase('Hello World'));
console.log('');

// Encoding
console.log('Encoding:');
console.log('base64Encode:', redstr.base64Encode('test payload'));
console.log('urlEncode:', redstr.urlEncode('test payload?'));
console.log('hexEncode:', redstr.hexEncode('test'));
console.log('htmlEntityEncode:', redstr.htmlEntityEncode('<script>alert(1)</script>'));
console.log('');

// Unicode
console.log('Unicode Transformations:');
console.log('homoglyphSubstitution:', redstr.homoglyphSubstitution('admin'));
console.log('zalgoText:', redstr.zalgoText('test'));
console.log('');

// Obfuscation
console.log('Obfuscation:');
console.log('leetspeak:', redstr.leetspeak('password'));
console.log('rot13:', redstr.rot13('Hello World'));
console.log('reverseString:', redstr.reverseString('Hello'));
console.log('jsStringConcat:', redstr.jsStringConcat('test'));
console.log('');

// Injection Testing
console.log('Injection Testing:');
console.log('sqlCommentInjection:', redstr.sqlCommentInjection('admin'));
console.log('xssTagVariations:', redstr.xssTagVariations('alert(1)'));
console.log('pathTraversal:', redstr.pathTraversal('etc/passwd'));
console.log('commandInjection:', redstr.commandInjection('ls'));
console.log('');

// Bot Detection
console.log('Bot Detection:');
console.log('randomUserAgent:', redstr.randomUserAgent());
console.log('acceptLanguageVariation:', redstr.acceptLanguageVariation('en-US'));
console.log('');

// Web Security
console.log('Web Security:');
console.log('apiEndpointVariation:', redstr.apiEndpointVariation('/api/users'));
console.log('sessionTokenVariation:', redstr.sessionTokenVariation('abc123'));
console.log('');

// Phishing
console.log('Phishing:');
console.log('domainTyposquat:', redstr.domainTyposquat('paypal.com'));
console.log('emailObfuscation:', redstr.emailObfuscation('test@example.com'));
console.log('');

// Shell
console.log('Shell Obfuscation:');
console.log('bashObfuscate:', redstr.bashObfuscate('echo hello'));
console.log('powershellObfuscate:', redstr.powershellObfuscate('Write-Host "test"'));
console.log('');
