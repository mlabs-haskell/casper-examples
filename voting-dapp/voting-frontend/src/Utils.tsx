
import {
  Keys,
  decodeBase64
} from "casper-js-sdk";



export function theKeys(): Keys.AsymmetricKey {
  // key of user-2 in `nctl-docker` 
  return parseSecretKey("MC4CAQAwBQYDK2VwBCIEIJ3WEDyVs7vJpLbBtrsqSeOBAZaX9q0lCiGKYtGzqXgF");
  
  // key of account on testnet
  // return parseSecretKey("MC4CAQAwBQYDK2VwBCIEIFQo20xQWRoFA0bRdVvLY6zpoiUkbeVPMihbP0zZ9rEg");

}

function parseSecretKey(encodedKey: string): Keys.AsymmetricKey {
  const privateKey = Keys.Ed25519.parsePrivateKey(
    decodeBase64(encodedKey));
  return Keys.Ed25519.parseKeyPair(
    Keys.Ed25519.privateToPublicKey(privateKey),
    privateKey
  )
}
