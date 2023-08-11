import { useState } from 'react';
import './App.css';
import { Proposals } from './Proposals';
import { IContractInfo, ICurrentKey } from './AppTypes'
import { Init } from './Init';
import { NewProposal } from './NewProposal';

declare global {
  interface Window {
    CasperWalletProvider: any;
  }
}

function App() {
  const [pubKey, setPubKey] = useState<ICurrentKey>({ pubKey: undefined });
  const [contractInfo, setContractInfo] = useState<IContractInfo | undefined>(undefined);

  const setKey = (keyHash: string) => {
    setPubKey({ pubKey: keyHash })
  }

  return (
    <div className="App">
      <p>Governor package hash: {contractInfo?.package_hash}</p>
      <p>Governor contract hash: {contractInfo?.contract_hash}</p>
      <p>Current pub key: {pubKey.pubKey}</p>
      <Init
        setKey={setKey}
        setContractInfo={setContractInfo}
      />
      <NewProposal pubKey={pubKey}/>
      <Proposals pubKey={pubKey} />
    </div>
  );
}

export default App;
