#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const output = process.argv[2] || path.resolve(__dirname, '..', 'charlie-mj-extension.pem');

if (fs.existsSync(output)) {
  console.log(`CRX signing key already exists: ${output}`);
  process.exit(0);
}

const { privateKey } = crypto.generateKeyPairSync('rsa', {
  modulusLength: 2048,
  publicKeyEncoding: { type: 'spki', format: 'pem' },
  privateKeyEncoding: { type: 'pkcs1', format: 'pem' }
});

fs.writeFileSync(output, privateKey, { encoding: 'utf8', mode: 0o600 });
console.log(`Created CRX signing key: ${output}`);
console.log('Keep this file private. It determines the Chrome extension identity.');
