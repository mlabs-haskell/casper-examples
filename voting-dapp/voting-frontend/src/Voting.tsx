import { ICurrentKey } from "./AppTypes";
import { governorContract, magicSleep, signDeploy, } from './CasperNetwork'
import { Vote } from "./GovernorClient";

export const Voting: React.FC<{
  pubKey: ICurrentKey, proposalId: number
}> = ({ pubKey, proposalId }) => {
  return (
    <div>
      <button onClick={() => {
        vote(pubKey, proposalId, Vote.Yea).catch(err => alert(err))
      }}>Yea</button>
      <button onClick={() => {
        vote(pubKey, proposalId, Vote.Nay).catch(err => alert(err))
      }}>Nay</button>
    </div>
  );
};

async function vote(iPubKey: ICurrentKey, proposalId: number, vote: Vote) {
  console.log(`Voting ${vote} for proposal ${proposalId}`);

  if (!iPubKey.pubKey) {
    throw new Error("Public key is missing. Is wallet connected?")
  };

  const keyHash = iPubKey.pubKey!;

  const deploy = governorContract.vote(proposalId, vote, keyHash);
  const signedDeploy = await signDeploy(deploy, keyHash);
  magicSleep(1000);
  const [success, failure] = await governorContract.putDeploy(signedDeploy, keyHash);
  if (failure) {
    let msg =
      failure!.error_message === "User error: 0"
        ? 'you voted on this proposal already'
        : failure!.error_message;

    msg = `Failed to vote: ${msg}`;
    console.error(msg);
    alert(msg);
  } else {
    console.log(`Voted successfully for ${proposalId}`);
    console.log({
      proposalId: proposalId,
      keyHash: keyHash,
      result: success
    });
    alert(`Voted successfully for ${proposalId}!`);
  }
}
