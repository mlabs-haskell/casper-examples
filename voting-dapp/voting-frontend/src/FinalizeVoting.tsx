import { ICurrentKey } from "./AppTypes";
import {
  governorContract,
  magicSleep,
  signDeploy
} from './CasperNetwork'

export const FinalizeVoting: React.FC<{
  pubKey: ICurrentKey, proposalId: number
}> = ({ pubKey, proposalId }) => {

  return (
    <div>
      <button onClick={() => {
        finalize(pubKey, proposalId).catch(err => alert(err))
      }}>Finish voting</button>
    </div>
  );
};

async function finalize(iPubKey: ICurrentKey, proposalId: number) {
  console.log(`Finalizing voting for proposal ${proposalId}`);

  if (!iPubKey.pubKey) {
    throw new Error("Public key is missing. Is wallet connected?")
  };

  const keyHash = iPubKey.pubKey!;

  // todo: should be extracted to some contract client
  const deploy = governorContract.finalizeVoting(proposalId, keyHash);
  const signedDeploy = await signDeploy(deploy, keyHash);
  magicSleep(1000);
  const [success, failure] = await governorContract.putDeploy(signedDeploy, keyHash);
  if (failure) {
    const msg = `Failed to finalize voting: ${failure!.error_message}`;
    console.error(msg);
    alert(msg);
  } else {
    console.log(`Voting finished successfully for ${proposalId}`);
    console.log({
      proposalId: proposalId,
      keyHash: keyHash,
      result: success
    });
    alert(`Voting finished successfully for ${proposalId}!`);
  }
}
