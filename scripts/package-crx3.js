#!/usr/bin/env node
'use strict';

const fs = require('fs');
const crypto = require('crypto');

function usage() {
  console.error('Usage: node package-crx3.js <private-key.pem> <extension.zip> <output.crx>');
  process.exit(1);
}

const [keyPath, zipPath, outputPath] = process.argv.slice(2);
if (!keyPath || !zipPath || !outputPath) usage();

if (!fs.existsSync(keyPath)) throw new Error(`Private key not found: ${keyPath}`);
if (!fs.existsSync(zipPath)) throw new Error(`Extension ZIP not found: ${zipPath}`);

const privateKeyPem = fs.readFileSync(keyPath);
const archive = fs.readFileSync(zipPath);
const privateKey = crypto.createPrivateKey(privateKeyPem);
const publicKey = crypto.createPublicKey(privateKey).export({ type: 'spki', format: 'der' });

// Chrome's CRX3 extension ID is the first 16 bytes of SHA-256(SPKI),
// rendered as 32 letters a-p. This is also the crx_id in SignedData.
const crxId = crypto.createHash('sha256').update(publicKey).digest().subarray(0, 16);

function varint(value) {
  const out = [];
  let n = BigInt(value);
  while (n >= 0x80n) {
    out.push(Number((n & 0x7fn) | 0x80n));
    n >>= 7n;
  }
  out.push(Number(n));
  return Buffer.from(out);
}

function bytesField(fieldNumber, value) {
  const data = Buffer.from(value);
  const tag = varint((BigInt(fieldNumber) << 3n) | 2n);
  return Buffer.concat([tag, varint(data.length), data]);
}

// SignedData { bytes crx_id = 1; }
const signedHeaderData = bytesField(1, crxId);

// CRX3 signs:
// "CRX3 SignedData\\0" + uint32LE(signedHeaderData.length) + signedHeaderData + ZIP
const signedMessage = Buffer.concat([
  Buffer.from('CRX3 SignedData\0', 'utf8'),
  (() => { const b = Buffer.alloc(4); b.writeUInt32LE(signedHeaderData.length, 0); return b; })(),
  signedHeaderData,
  archive,
]);

// Chromium's current CRX3 verifier accepts the RSA/SHA-256 proof using PKCS#1 v1.5 padding.
const signature = crypto.sign('sha256', signedMessage, {
  key: privateKey,
  padding: crypto.constants.RSA_PKCS1_PADDING,
});

// Verify the signature before writing the package. This catches malformed
// CRX files during CI instead of producing an artifact Chrome will reject.
const verified = crypto.verify('sha256', signedMessage, {
  key: publicKey,
  format: 'der',
  type: 'spki',
  padding: crypto.constants.RSA_PKCS1_PADDING,
}, signature);

if (!verified) {
  throw new Error('CRX3 self-verification failed: the RSA/SHA-256 signature is invalid.');
}

// AsymmetricKeyProof { bytes public_key = 1; bytes signature = 2; }
const proof = Buffer.concat([
  bytesField(1, publicKey),
  bytesField(2, signature),
]);

// CrxFileHeader {
//   repeated AsymmetricKeyProof sha256_with_rsa = 2;
//   bytes signed_header_data = 10000;
// }
const header = Buffer.concat([
  bytesField(2, proof),
  bytesField(10000, signedHeaderData),
]);

const prefix = Buffer.alloc(12);
prefix.write('Cr24', 0, 'ascii');
prefix.writeUInt32LE(3, 4);
prefix.writeUInt32LE(header.length, 8);

fs.writeFileSync(outputPath, Buffer.concat([prefix, header, archive]));
console.log('CRX3 signature verification: OK');
console.log(`Created CRX3: ${outputPath}`);
console.log(`CRX ID: ${crxId.toString('hex')}`);
console.log(`CRX size: ${fs.statSync(outputPath).size} bytes`);
