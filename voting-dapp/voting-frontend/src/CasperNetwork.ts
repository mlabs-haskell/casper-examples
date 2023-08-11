import { CLPublicKey, DeployUtil } from "casper-js-sdk";
import { IDeployedGovernor, IProposals } from "./AppTypes";
import axios from "axios";
import { theKeys } from "./Utils";
import { NETWORK_NAME, NODE_URL, QUERY_SERVICE_URL, USE_CASPER_WALLET } from "./Settings";
import { Governor } from "./GovernorClient";

const CasperWalletProvider = window.CasperWalletProvider;
export const walletProvider = CasperWalletProvider();

export const governorContract = new Governor(NODE_URL, NETWORK_NAME);

export async function queryDeployedGovernor(): Promise<IDeployedGovernor> {
  console.log("Query deployed governor")
  let resp = await axios.get<IDeployedGovernor>(QUERY_SERVICE_URL + "/governor")
  return resp.data
}

export async function queryProposals(): Promise<IProposals> {
  try {
    console.log("Query proposals")
    const resp = await axios.get<IProposals>(QUERY_SERVICE_URL + "/proposals");
    return resp.data
  } catch (e) {
    console.error(e)
    return { proposals: [] }
  }
}

/*! if deploy timestamp will be later in time than node current time,
      node will treat such deploy as invalid. This happened to me when
      sending deploys to some public nodes on testnet.
  */
export const magicSleep = (ms: number) => new Promise(r => {
  console.log("Magic sleep...")
  setTimeout(r, ms)
});

export async function signDeploy(deploy: DeployUtil.Deploy, keyHash: string): Promise<DeployUtil.Deploy> {
  if (USE_CASPER_WALLET) {
    return signWithBrowserExtension(deploy, keyHash);
  } else {
    return signWithPredefinedKey(deploy);
  }
}

async function signWithBrowserExtension(deploy: DeployUtil.Deploy, keyHash: string) {
  const deployJson = DeployUtil.deployToJson(deploy);
  try {
    const signature = await walletProvider.sign(JSON.stringify(deployJson), keyHash);
    if (signature.cancelled) {
      throw new Error("Sign cancelled")
    };
    return DeployUtil.setSignature(
      deploy,
      signature.signature,
      CLPublicKey.fromHex(keyHash)
    );
  } catch (e) {
    console.error(`Could not sing deploy. Error: ${e}`);
    throw new Error(`Could not sing deploy. Error: ${e}`);
  }
}

async function signWithPredefinedKey(deploy: DeployUtil.Deploy) {
  return deploy.sign([theKeys()])
}
