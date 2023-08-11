export type ICurrentKey = {
  pubKey?: string
}

export interface IProposals {
  proposals: IProposal[]
}
export enum Status {
  Active = "Active",
  Finished = "Finished"
}
export interface IProposal {
  id: number,
  statement: string,
  yea: number,
  nay: number,
  status: Status
}

export interface IDeployedGovernor {
  package_key: string,
  package_hash: string,
}

export interface IContractInfo {
  package_hash: string,
  contract_hash: string
}