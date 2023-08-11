import { useState } from "react";
import { ICurrentKey } from "./AppTypes";
import { governorContract, magicSleep, signDeploy } from "./CasperNetwork";
import { CLValueBuilder, RuntimeArgs, encodeBase16 } from "casper-js-sdk";

export const NewProposal: React.FC<{ pubKey: ICurrentKey }> = ({ pubKey }) => {
  const [proposal, setProposal] = useState<string>("");
  // Contract address to call if voted "Yes"
  // Note: Odra contracts require package hash to perform calls
  const [contractAddress, setContractAddress] = useState<string>("");

  // Entrypoint we want to call if voted "Yes"
  const [contractEntrypoint, setContractEntrypoint] = useState<string>("");

  /* Argument(s) for Entrypoint. Sort of hardcoded now for exact contract.
     Ideally we should be able to build required `RuntimeArgs` from whatever data
     user provides from the browser side.
  */
  const [addAmount, setAddAmount] = useState<string>("");

  async function submitProposal(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    if (!pubKey.pubKey) {
      throw new Error("Public key is missing. Is wallet connected?")
    };

    e.preventDefault();
    if (!proposal || !contractAddress || !contractEntrypoint || !addAmount) {
      alert("Please fill in proposal form");
    } else {
      await submit(proposal, contractAddress, contractEntrypoint, addAmount, pubKey.pubKey!);
      setProposal("");
    }
  };

  return (
    <div className="NewProposal">
      <form>
        <input
          placeholder="Proposal text"
          value={proposal}
          onChange={e => { setProposal(e.target.value) }} />
        <input
          placeholder="contract package hash"
          value={contractAddress}
          onChange={e => { setContractAddress(e.target.value) }} />
        <input
          placeholder="contract entry point"
          value={contractEntrypoint}
          onChange={e => { setContractEntrypoint(e.target.value) }} />
        <input
          placeholder="arguments"
          value={addAmount}
          onChange={e => { setAddAmount(e.target.value) }} />
        <button onClick={e => {
          submitProposal(e).catch(err => alert(err))
        }}>Add Proposal</button>
      </form>
    </div>
  );
};

async function submit(
  proposal: string,
  contractAddress: string,
  contractEntrypoint: string,
  addAmount: string,
  keyHash: string) {


  const entrypointRtArgs = RuntimeArgs.fromMap({
    add_amount: CLValueBuilder.i32(addAmount)
  });

  const serializedArgs = encodeBase16(entrypointRtArgs.toBytes().unwrap());

  const callData = CLValueBuilder.tuple2(
    [CLValueBuilder.string(contractAddress),
    CLValueBuilder.tuple2(
      [CLValueBuilder.string(contractEntrypoint),
      CLValueBuilder.string(serializedArgs)]
    )]

  );

  const deploy = governorContract.newProposal(proposal, keyHash, callData);
  const signedDeploy = await signDeploy(deploy, keyHash);
  magicSleep(1000);
  const [success, failure] = await governorContract.putDeploy(signedDeploy, keyHash);
  if (failure) {
    let msg = failure!.error_message === "User error: 0" ? 'you voted already' : failure!.error_message;
    msg = `Failed to submit proposal: ${msg}`;
    console.error(msg);
    alert(msg);
  } else {
    console.log(`Proposal created`);
    alert(`Proposal created!`);

    console.log({
      keyHash: keyHash,
      result: success
    });
  }
}
