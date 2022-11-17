//! This contract implements voting procedure for Eurovision Song Contest
//!  backed by storage on blockchain
//!
//!
use itertools::enumerate;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use std::collections::HashMap;
use std::hash::Hash;
use std::string::String;
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct EscVoting {
    scoreboard: HashMap<String, u64>,
    voting_history: HashMap<String, Vec<String>>,
}

#[near_bindgen]
impl EscVoting {
    #[init]
    pub fn new() -> Self {
        Self {
            scoreboard: HashMap::new(),
            voting_history: HashMap::new(),
        }
    }

    pub fn get_scoreboard(&self) -> HashMap<String, u64> {
        return self.scoreboard.clone();
    }

    pub fn get_voting_by_name(&self, name: String) -> Vec<String> {
        self.voting_history.get(&name).unwrap().clone()
    }

    pub fn get_list_of_voters(&self) -> Vec<String> {
        return self.voting_history.keys().cloned().collect();
    }

    pub fn is_voter_exist(&self, voter: String) -> bool {
        self.voting_history.get(&voter) != None
    }

    pub fn update_scoreboard_with_list(&mut self, input_list: Vec<String>, voter: String) {
        let list_to_insert = input_list.clone();
        let points: [u64; 10] = [12, 10, 8, 7, 6, 5, 4, 3, 2, 1];
        for (idx, country) in enumerate(list_to_insert) {
            if self.scoreboard.contains_key(&country) {
                *self.scoreboard.get_mut(&country).unwrap() += points[idx];
            } else {
                self.scoreboard.insert(country, points[idx]);
            }
        }
        self.voting_history.insert(voter, input_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;
    use std::any::type_name;
    // fn to_valid_account(account: &str) -> ValidAccountId {
    //     ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    // }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    fn type_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    pub fn test_get_scoreboard() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        assert_eq!(
            type_of(&contract.get_scoreboard()),
            "std::collections::hash::map::HashMap<alloc::string::String, u64>"
        )
    }

    #[test]
    pub fn test_list_of_voters() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        assert_eq!(
            type_of(&contract.get_list_of_voters()),
            "alloc::vec::Vec<alloc::string::String>"
        )
    }

    #[test]
    pub fn test_update_scoreboard_with_list() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let voting_list = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ];
        contract.update_scoreboard_with_list(voting_list, String::from("new_voter"));
        assert_eq!(contract.scoreboard.len(), 10)
    }

    #[test]
    pub fn test_get_voting_by_name() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let voter_name = String::from("new_voter");
        let voting_list = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ];
        contract
            .voting_history
            .insert(voter_name.clone(), voting_list.clone());
        assert_eq!(contract.get_voting_by_name(voter_name), voting_list)
    }

    #[test]
    pub fn test_is_voter_exist() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let voter_name = String::from("new_voter");
        let voting_list = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ];
        contract
            .voting_history
            .insert(voter_name.clone(), voting_list.clone());
        assert_eq!(contract.is_voter_exist(String::from("new_voter")), true)
    }
}
