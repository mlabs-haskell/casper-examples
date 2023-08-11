use contracts::types as contract;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Finished,
}

#[derive(Serialize, Deserialize)]
pub struct ProposalDTO {
    pub id: u64,
    pub statement: String,
    pub yea: u32,
    pub nay: u32,
    pub status: Status,
}

impl From<contract::Proposal> for ProposalDTO {
    fn from(p: contract::Proposal) -> Self {
        ProposalDTO {
            id: p.id,
            statement: p.statement,
            yea: p.yea,
            nay: p.nay,
            // todo: is there a better automatic way to do this w/o adding serde derivations to contract::Status?
            status: match p.status {
                contract::Status::Active => Status::Active,
                contract::Status::Finished => Status::Finished,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProposalsDTO {
    proposals: Vec<ProposalDTO>,
}

impl ProposalsDTO {
    pub fn empty() -> Self {
        ProposalsDTO {
            proposals: Vec::new(),
        }
    }
    pub fn add(&mut self, proposal: ProposalDTO) {
        self.proposals.push(proposal)
    }
}
