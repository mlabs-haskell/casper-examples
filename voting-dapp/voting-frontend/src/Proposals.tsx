import { useEffect, useState } from "react";
import { ICurrentKey, IProposals, Status } from './AppTypes'
import { Voting } from "./Voting";
import { queryProposals } from "./CasperNetwork";
import { FinalizeVoting } from "./FinalizeVoting";


export const Proposals: React.FC<{ pubKey: ICurrentKey }> = ({ pubKey }) => {
  const [proposals, setProposals] = useState<IProposals>({ proposals: [] });

  useEffect(() => {
    queryProposals().then(setProposals)
  }, []);

  return (
    <div className="section__proposals">
      <p>Proposals:</p>
      {proposals.proposals.length ? <ul className="proposals">
        {proposals.proposals.map(p => (
          <li key={p.id}>
            <p>ID: {p.id} | {p.statement} | yea: {p.yea} | nay: {p.nay}</p>
            {(p.status === Status.Active) ?
              <div>
                <Voting pubKey={pubKey} proposalId={p.id} />
                <FinalizeVoting pubKey={pubKey} proposalId={p.id} />
              </div>
              : <p>Voting finished</p>
            }
          </li>
        ))}
      </ul> : <div>No proposals found yet</div>}
    </div>
  );
};
