import { ContractWASM, CEP18Client, InstallArgs, EVENTS_MODE } from 'casper-cep18-js-client';
import { CLPublicKey, CasperClient } from 'casper-js-sdk';
import { AsymmetricKey } from 'casper-js-sdk/dist/lib/Keys';

const DEPLOY_GAS_PRICE = 330560983230;

const tokenInfo: InstallArgs = {
  name: 'TEST CEP18',
  symbol: 'TFT',
  decimals: 9,
  totalSupply: 200_000_000_000,
  eventsMode: EVENTS_MODE.CES,
  enableMintAndBurn: true
};

export async function deployTokensContract(
  nodeUrl: string,
  networkName: string,
  adminKeys: AsymmetricKey
): Promise<CEP18Client> {
  const cep18 = new CEP18Client(nodeUrl, networkName);
  const casperClient = new CasperClient(nodeUrl);

  const existingCep18Hash = await findKey(
    casperClient,
    adminKeys.publicKey,
    `cep18_contract_hash_${tokenInfo.name}`) as `hash-${string}`;
  if (existingCep18Hash) {
    console.log(`Fungible tokens contract already deployed`);
    cep18.setContractHash(existingCep18Hash);
    return cep18
  }

  console.log(`Deploying fungible tokens contract...`);
  const installDeploy = cep18.install(
    ContractWASM, // Contract wasm
    tokenInfo,
    DEPLOY_GAS_PRICE,
    adminKeys.publicKey,
    networkName,
    [adminKeys]
  );

  await installDeploy.send(nodeUrl);
  const installRes = await casperClient.nodeClient.waitForDeploy(installDeploy);
  console.log(installRes.execution_results[0].result);
  const installFailure = installRes.execution_results[0].result.Failure;
  if (installFailure) {
    throw new Error(`Failed to instal voting tokens contract: ${installFailure.error_message}`)
  }

  const cep18Hash =
    await findKey(
      casperClient,
      adminKeys.publicKey,
      `cep18_contract_hash_${tokenInfo.name}`) as `hash-${string}`;
  if (!cep18Hash) {
    throw new Error(`Something went wrong with fungible tokens contract.
     Contract hash in undefined after installation`)
  }
  cep18.setContractHash(cep18Hash);
  cep18.balanceOf
  console.log(`Deploying fungible tokens contract deployed`);
  return cep18

}

export async function findKey(
  casperClient: CasperClient,
  contractAccount: CLPublicKey, contractKey: string): Promise<string | undefined> {
  const rootHash = await casperClient.nodeClient.getStateRootHash()
  const accountHash = contractAccount.toAccountHashStr()
  const state = await casperClient.nodeClient
    .getBlockState(rootHash, accountHash, [])
  return state
    .Account
    ?.namedKeys
    .find(key => key.name === contractKey)
    ?.key
}
