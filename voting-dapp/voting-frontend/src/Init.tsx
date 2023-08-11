import { IContractInfo } from './AppTypes'
import { governorContract, queryDeployedGovernor, walletProvider } from './CasperNetwork';
import { USE_CASPER_WALLET } from './Settings';
import { theKeys } from './Utils';

/*
 Init app state required to call contracts:
 - connects to wallet extension or parse predefined keys from base64
 - quires package hash fro query service id finds contract hash 
   (coz JS SDK needs contract hash to call contract, not package hash like Odra) 
*/
export const Init: React.FC<{
  setKey: (keyHash: string) => void,
  setContractInfo: (info: IContractInfo) => void,
}> = ({ setKey, setContractInfo }) => {

  return (
    <button onClick={() => { init().catch(err => alert(err)) }}>
      Init
    </button>
  )

  async function init() {
    // setting pub key hash
    const keyHash: string = await currentPubKeyHash();
    setKey(keyHash);

    // setting package and contract hash
    const deployedGovernor = await queryDeployedGovernor();
    const packageHash = deployedGovernor.package_hash;
    const rootHash = await governorContract.casperClient.nodeClient.getStateRootHash();
    let contractHash = await governorContract.casperClient.nodeClient
      .getBlockState(rootHash, packageHash, [])
      .then(p => p.ContractPackage?.versions[0].contractHash);

    if (!contractHash) {
      throw new Error(`Failed to find contract hash for package hash ${packageHash}`)
    }

    contractHash = contractHash!.replace("contract-", "hash-");
    governorContract.setContractHash(contractHash);
    setContractInfo({
      package_hash: packageHash,
      contract_hash: contractHash
    })
  }
};

async function currentPubKeyHash() {
  if (USE_CASPER_WALLET) {
    const connected: boolean = await walletProvider.requestConnection();
    if (!connected) {
      throw new Error("Could not connect to wallet")
    }
    const keyHash: string = await walletProvider.getActivePublicKey();
    return keyHash
  } else {
    return theKeys().publicKey.toHex();
  }
}