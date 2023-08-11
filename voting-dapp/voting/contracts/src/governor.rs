use odra::{
    execution_error,
    types::{Address, CallArgs, OdraType},
    List, Mapping, Sequence, UnwrapOrRevert, Variable,
};

use crate::types::*;
use hex::decode;
use std::str::FromStr;

type PackageHash = String;
type EntryPoint = String;
type SerializedArgs = String;
// for some reason Tuple3 does not compiled
type ContractCallData = (PackageHash, (EntryPoint, SerializedArgs));
#[odra::module]
pub struct Governor {
    ids_gen: Sequence<ProposalId>,
    admin: Variable<Address>,
    name: Variable<String>,
    proposals: Mapping<ProposalId, Proposal>,
    voters_registry: Mapping<(ProposalId, Address), Vote>,
    all_proposals: List<ProposalId>,
    contract_call_data: Mapping<ProposalId, ContractCallData>,
}

execution_error! {
  pub enum Error {
      AddressAlreadyVoted => 0,
      ProposalDoesNotExist => 1,
      VotingFinished => 2,
      ContractCallDataNotFound => 3,
  }
}

// TODO: maybe some `delete_proposal` method can be added to delete malformed proposal
#[odra::module]
impl Governor {
    #[odra(init)]
    pub fn init(&mut self, name: String) {
        self.admin.set(odra::contract_env::caller());
        self.name.set(name);
    }

    // All pub functions Odra converts to Contract entry points
    pub fn get_name(&self) -> String {
        // For `getters` livenet Odra client can query node via JSON RPC calls
        self.name.get().unwrap_or_revert()
    }

    pub fn get_call_data(&self, proposal_id: ProposalId) -> ContractCallData {
        self.contract_call_data
            .get(&proposal_id)
            .unwrap_or_revert_with(Error::ContractCallDataNotFound)
    }

    pub fn get_proposal(&mut self, proposal_id: ProposalId) -> Proposal {
        self.proposals
            .get(&proposal_id)
            .unwrap_or_revert_with(Error::ProposalDoesNotExist)
    }

    pub fn new_proposal(&mut self, statement: String, call_data: ContractCallData) {
        let next_id = self.ids_gen.next_value();
        self.proposals
            .set(&next_id, Proposal::new(next_id, statement));
        self.all_proposals.push(next_id);
        self.contract_call_data.set(&next_id, call_data);
    }

    pub fn last_proposal_id(&self) -> ProposalId {
        self.ids_gen.get_current_value()
    }

    pub fn vote_for(&mut self, proposal_id: ProposalId) {
        self.vote(proposal_id, Vote::Yea)
    }

    pub fn vote_against(&mut self, proposal_id: ProposalId) {
        self.vote(proposal_id, Vote::Nay)
    }

    pub fn finalize_voting(&mut self, proposal_id: ProposalId) {
        let mut proposal = self.get_proposal(proposal_id);
        if let Status::Finished = proposal.status {
            odra::contract_env::revert(Error::VotingFinished);
        }

        if proposal.voted_yes() {
            self.call_voted_contract(proposal_id)
        }

        proposal.status = Status::Finished;
        self.proposals.set(&proposal_id, proposal);
    }

    fn call_voted_contract(&mut self, proposal_id: ProposalId) {
        let contract_data = self
            .contract_call_data
            .get(&proposal_id)
            .unwrap_or_revert_with(Error::ContractCallDataNotFound);
        let package_hash_addr = Address::from_str(&contract_data.0).unwrap();
        let entrypoint = contract_data.1 .0;

        //todo: there should be a way to accept call args as array of bytes
        // and avoid hex decoding step here. 
        // Need to figure out appropriate Odra type for the argument.
        let call_args =
            CallArgs::deserialize(&decode(&contract_data.1 .1).unwrap()).unwrap_or_revert();
        odra::call_contract::<()>(package_hash_addr, &entrypoint, &call_args, None);
    }

    fn vote(&mut self, proposal_id: ProposalId, vote: Vote) {
        let caller = odra::contract_env::caller();
        let registry_key = (proposal_id, caller);

        // check if Address already voted
        match self.voters_registry.get(&registry_key) {
            Some(_) => odra::contract_env::revert(Error::AddressAlreadyVoted),
            None => self.voters_registry.set(&registry_key, vote.clone()),
        }

        let proposal = self.get_proposal(proposal_id);
        if let Status::Finished = proposal.status {
            odra::contract_env::revert(Error::VotingFinished);
        }

        let proposal = match vote {
            Vote::Yea => Proposal {
                yea: proposal.yea + 1,
                ..proposal
            },
            Vote::Nay => Proposal {
                nay: proposal.nay + 1,
                ..proposal
            },
        };
        self.proposals.set(&proposal_id, proposal)
    }
}

#[cfg(test)]
mod tests {
    use odra::{test_env, types::Address};

    use crate::{governor::Error, types::Status, GovernorRef};

    use super::{ContractCallData, GovernorDeployer, Proposal};

    const TEST_NAME: &str = "Test Name";

    fn deploy_new() -> (Address, GovernorRef) {
        let admin = test_env::get_account(0);
        test_env::set_caller(admin);
        let gov_contract = GovernorDeployer::init(TEST_NAME.to_string());
        (admin, gov_contract)
    }

    #[test]
    fn deploy() {
        let (_admin, mut contract) = deploy_new();
        assert_eq!(TEST_NAME.to_string(), contract.get_name());
        odra::test_env::assert_exception(Error::ProposalDoesNotExist, || {
            contract.get_proposal(0);
        });
    }

    #[test]
    fn create_and_get_proposal() {
        let dummy_call_data: ContractCallData = (
            String::from("dummy1"),
            (String::from("dummy2"), String::from("dummy3")),
        );
        let (_admin, mut contract) = deploy_new();
        let user_1 = test_env::get_account(1);

        test_env::set_caller(user_1);
        let statement = String::from("Do Something");
        contract.new_proposal(statement.clone(), dummy_call_data);

        let expected = Proposal {
            id: 0,
            statement: statement,
            yea: 0,
            nay: 0,
            status: Status::Active,
        };

        assert_eq!(expected, contract.get_proposal(0));
        odra::test_env::assert_exception(Error::ProposalDoesNotExist, || {
            contract.get_proposal(1);
        })
    }

    #[test]
    fn vote() {
        let dummy_call_data: ContractCallData = (
            String::from("dummy1"),
            (String::from("dummy2"), String::from("dummy3")),
        );
        let (_admin, mut contract) = deploy_new();
        let user_1 = test_env::get_account(1);
        let user_2 = test_env::get_account(2);
        contract.new_proposal(String::from("Some proposal"), dummy_call_data);

        test_env::set_caller(user_1);
        contract.vote_for(0);
        odra::test_env::assert_exception(Error::AddressAlreadyVoted, || contract.vote_against(0));
        odra::test_env::assert_exception(Error::AddressAlreadyVoted, || contract.vote_against(0));

        test_env::set_caller(user_2);
        contract.vote_against(0);

        let expected = Proposal {
            id: 0,
            statement: String::from("Some proposal"),
            yea: 1,
            nay: 1,
            status: Status::Active,
        };
        assert_eq!(expected, contract.get_proposal(0));

        odra::test_env::assert_exception(Error::ProposalDoesNotExist, || contract.vote_against(1));

        odra::test_env::assert_exception(Error::ProposalDoesNotExist, || contract.vote_for(1));
    }

    #[test]
    fn proposal_ids() {
        let dummy_call_data: ContractCallData = (
            String::from("dummy1"),
            (String::from("dummy2"), String::from("dummy3")),
        );
        let (_admin, mut contract) = deploy_new();
        let n = 3;
        for idx in 0..=n {
            println!("V: {}", idx);
            contract.new_proposal(format!("Proposal #{}", idx), dummy_call_data.clone())
        }

        assert_eq!(3, contract.last_proposal_id())
    }
}
