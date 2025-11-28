const test = require('ava');
const { existsSync } = require('fs');
const { join } = require('path');

// Test that verifies the native bindings module exists and is loadable
test('native bindings module exists', (t) => {
  const platform = process.platform;
  const arch = process.arch;
  
  let expectedFile;
  
  if (platform === 'linux') {
    if (arch === 'x64') {
      expectedFile = 'redstr.linux-x64-gnu.node';
    } else if (arch === 'arm64') {
      expectedFile = 'redstr.linux-arm64-gnu.node';
    } else {
      t.pass('Skipping platform check for ' + platform + '-' + arch);
      return;
    }
  } else if (platform === 'darwin') {
    if (arch === 'x64') {
      expectedFile = 'redstr.darwin-x64.node';
    } else if (arch === 'arm64') {
      expectedFile = 'redstr.darwin-arm64.node';
    }
  } else if (platform === 'win32') {
    if (arch === 'x64') {
      expectedFile = 'redstr.win32-x64-msvc.node';
    } else if (arch === 'ia32') {
      expectedFile = 'redstr.win32-ia32-msvc.node';
    } else if (arch === 'arm64') {
      expectedFile = 'redstr.win32-arm64-msvc.node';
    }
  }
  
  if (expectedFile) {
    const modulePath = join(__dirname, '..', expectedFile);
    const exists = existsSync(modulePath);
    t.true(exists, `Native module should exist at ${modulePath}`);
  } else {
    t.pass('No expected file for platform: ' + platform + '-' + arch);
  }
});

// Test that the native module can be loaded
test('native bindings can be loaded', (t) => {
  let nativeModule;
  
  // This should not throw
  t.notThrows(() => {
    nativeModule = require('../index');
  });
  
  t.truthy(nativeModule, 'Native module should be loaded');
});

// Test that all expected exports exist
test('all transformation functions are exported', (t) => {
  const redstr = require('../index');
  
  // Case transformations
  t.is(typeof redstr.randomizeCapitalization, 'function');
  t.is(typeof redstr.alternateCase, 'function');
  t.is(typeof redstr.caseSwap, 'function');
  t.is(typeof redstr.inverseCase, 'function');
  t.is(typeof redstr.toCamelCase, 'function');
  t.is(typeof redstr.toSnakeCase, 'function');
  t.is(typeof redstr.toKebabCase, 'function');
  
  // Encoding transformations
  t.is(typeof redstr.base64Encode, 'function');
  t.is(typeof redstr.urlEncode, 'function');
  t.is(typeof redstr.hexEncode, 'function');
  t.is(typeof redstr.hexEncodeMixed, 'function');
  t.is(typeof redstr.htmlEntityEncode, 'function');
  t.is(typeof redstr.mixedEncoding, 'function');
  
  // Unicode transformations
  t.is(typeof redstr.homoglyphSubstitution, 'function');
  t.is(typeof redstr.unicodeVariations, 'function');
  t.is(typeof redstr.zalgoText, 'function');
  t.is(typeof redstr.spaceVariants, 'function');
  t.is(typeof redstr.unicodeNormalizeVariants, 'function');
  
  // Injection transformations
  t.is(typeof redstr.sqlCommentInjection, 'function');
  t.is(typeof redstr.xssTagVariations, 'function');
  t.is(typeof redstr.nullByteInjection, 'function');
  t.is(typeof redstr.pathTraversal, 'function');
  t.is(typeof redstr.commandInjection, 'function');
  t.is(typeof redstr.mongodbInjection, 'function');
  t.is(typeof redstr.couchdbInjection, 'function');
  t.is(typeof redstr.dynamodbObfuscate, 'function');
  t.is(typeof redstr.nosqlOperatorInjection, 'function');
  t.is(typeof redstr.sstiInjection, 'function');
  t.is(typeof redstr.sstiFrameworkVariation, 'function');
  t.is(typeof redstr.sstiSyntaxObfuscate, 'function');
  
  // Obfuscation transformations
  t.is(typeof redstr.leetspeak, 'function');
  t.is(typeof redstr.rot13, 'function');
  t.is(typeof redstr.vowelSwap, 'function');
  t.is(typeof redstr.doubleCharacters, 'function');
  t.is(typeof redstr.reverseString, 'function');
  t.is(typeof redstr.whitespacePadding, 'function');
  t.is(typeof redstr.jsStringConcat, 'function');
  
  // Phishing transformations
  t.is(typeof redstr.domainTyposquat, 'function');
  t.is(typeof redstr.advancedDomainSpoof, 'function');
  t.is(typeof redstr.emailObfuscation, 'function');
  t.is(typeof redstr.urlShorteningPattern, 'function');
  
  // Bot detection transformations
  t.is(typeof redstr.randomUserAgent, 'function');
  t.is(typeof redstr.tlsFingerprintVariation, 'function');
  t.is(typeof redstr.http2HeaderOrder, 'function');
  t.is(typeof redstr.acceptLanguageVariation, 'function');
  t.is(typeof redstr.cloudflareChallengeVariation, 'function');
  
  // Cloudflare transformations
  t.is(typeof redstr.cloudflareTurnstileVariation, 'function');
  t.is(typeof redstr.cloudflareChallengeResponse, 'function');
  t.is(typeof redstr.tlsHandshakePattern, 'function');
  t.is(typeof redstr.canvasFingerprintVariation, 'function');
  t.is(typeof redstr.webglFingerprintObfuscate, 'function');
  t.is(typeof redstr.fontFingerprintConsistency, 'function');
  
  // Web security transformations
  t.is(typeof redstr.httpHeaderVariation, 'function');
  t.is(typeof redstr.apiEndpointVariation, 'function');
  t.is(typeof redstr.graphqlObfuscate, 'function');
  t.is(typeof redstr.sessionTokenVariation, 'function');
  t.is(typeof redstr.graphqlVariableInjection, 'function');
  t.is(typeof redstr.graphqlIntrospectionBypass, 'function');
  t.is(typeof redstr.jwtHeaderManipulation, 'function');
  t.is(typeof redstr.jwtPayloadObfuscate, 'function');
  t.is(typeof redstr.jwtAlgorithmConfusion, 'function');
  t.is(typeof redstr.jwtSignatureBypass, 'function');
  
  // Shell transformations
  t.is(typeof redstr.powershellObfuscate, 'function');
  t.is(typeof redstr.bashObfuscate, 'function');
  t.is(typeof redstr.envVarObfuscate, 'function');
  t.is(typeof redstr.filePathObfuscate, 'function');
});

// Test that functions can be called and return expected types
test('functions return strings when called', (t) => {
  const redstr = require('../index');
  
  // Test a sampling of functions
  t.is(typeof redstr.base64Encode('test'), 'string');
  t.is(typeof redstr.leetspeak('test'), 'string');
  t.is(typeof redstr.rot13('test'), 'string');
  t.is(typeof redstr.randomUserAgent(), 'string');
  t.is(typeof redstr.sqlCommentInjection('test'), 'string');
});
