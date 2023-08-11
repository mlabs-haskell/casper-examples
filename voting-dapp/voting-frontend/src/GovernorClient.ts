import { CLPublicKey, CLTuple2, CLValueBuilder, CasperClient, Contracts, DeployUtil, RuntimeArgs } from "casper-js-sdk";

const PROPOSAL_GAS = "5000000000";
const VOTE_GAS = "2500000000";
const FINALIZE_GAS = "3000000000";

export enum Vote {
  Yea = '"yea"',
  Nay = '"nay"'
}

export class Governor {

  private networkName: string;
  private contractClient: Contracts.Contract;
  public casperClient: CasperClient;


  constructor(nodeUrl: string, networkName: string) {
    this.casperClient = new CasperClient(nodeUrl);
    this.contractClient = new Contracts.Contract(this.casperClient);
    this.networkName = networkName;
  }

  newProposal(proposal: string, pubKeyHash: string, callData: CLTuple2): DeployUtil.Deploy {
    return this.contractClient.callEntrypoint(
      "new_proposal",
      RuntimeArgs.fromMap({
        statement: CLValueBuilder.string(proposal),
        call_data: callData
      }),
      CLPublicKey.fromHex(pubKeyHash),
      this.networkName,
      PROPOSAL_GAS,
    );
  }

  vote(proposalId: number, vote: Vote, keyHash: string) {
    const endpoint = vote === Vote.Yea ? "vote_for" : "vote_against";

    return this.contractClient.callEntrypoint(
      endpoint,
      RuntimeArgs.fromMap({ proposal_id: CLValueBuilder.u64(proposalId) }),
      CLPublicKey.fromHex(keyHash),
      this.networkName,
      VOTE_GAS,
    );
  }

  finalizeVoting(proposalId: number, pubKeyHash: string): DeployUtil.Deploy {
    return this.contractClient.callEntrypoint(
      "finalize_voting",
      RuntimeArgs.fromMap({ proposal_id: CLValueBuilder.u64(proposalId) }),
      CLPublicKey.fromHex(pubKeyHash),
      this.networkName,
      FINALIZE_GAS,
    );
  }

  async putDeploy(signedDeploy: DeployUtil.Deploy, keyHash: string) {
    console.log(`Sending deploy to the network "${this.networkName}"...`)
    const deployHash = await this.casperClient.putDeploy(signedDeploy);
    console.log(`Deploy hash: ${deployHash}`);
    console.log(`Waiting for deployment to finish...`);
    const result = await this.casperClient.nodeClient.waitForDeploy(signedDeploy);
    const failure = result.execution_results[0].result.Failure;
    const success = result.execution_results[0].result.Success;
    return [success, failure]
  }

  setContractHash(contractHash: string) {
    this.contractClient.setContractHash(contractHash)
  }
}
