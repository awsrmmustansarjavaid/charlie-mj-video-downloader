const fs = require("fs");
const crypto = require("crypto");

const pemPath = process.argv[2];
if (!pemPath) {
  console.error("Usage: node extension-id.js <private-key.pem>");
  process.exit(1);
}

const publicKey = crypto.createPublicKey(fs.readFileSync(pemPath));
const spkiDer = publicKey.export({ type: "spki", format: "der" });
const digest = crypto.createHash("sha256").update(spkiDer).digest();

let id = "";
for (const byte of digest) {
  id += String.fromCharCode("a".charCodeAt(0) + ((byte >> 4) & 0x0f));
  id += String.fromCharCode("a".charCodeAt(0) + (byte & 0x0f));
}
process.stdout.write(id);
