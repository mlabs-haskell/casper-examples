import * as fs from 'fs';

import {
  Keys
} from "casper-js-sdk";

export function readWasm(path: fs.PathOrFileDescriptor): Uint8Array {
  return new Uint8Array(fs.readFileSync(path))
}

export function readKeys(path: String): Keys.AsymmetricKey {
  return Keys.Ed25519.parseKeyFiles(
    path + "/public_key.pem",
    path + "/secret_key.pem")
}

// export function falureReason()
