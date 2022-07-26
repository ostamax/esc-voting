//! This contract implements voting procedure for Eurovision Song Contest
//!  backed by storage on blockchain
//! 
//! 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use std::hash::Hash;
use std::string::String;
use std::collections::HashMap;
use itertools::enumerate;
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct EscVoting {
    scoreboard: HashMap<String, u64>,
    list_of_voters: Vec<String>,
    voting_history: HashMap<String, Vec<String>>,
}

#[near_bindgen]
impl EscVoting {

    #[init]
    pub fn new() -> Self {
        Self { 
            scoreboard: HashMap::from([
                (String::from("Ukraine"), 0),
                (String::from("Denmark"), 0),
                (String::from("The Netherlands"), 0),
                (String::from("Greece"), 0),
                (String::from("Cyprus"), 0),
                (String::from("Romania"), 0),
                (String::from("Croatia"), 0),
                (String::from("Portugal"), 0),
                (String::from("Sweden"), 0),
                (String::from("Switzerland"), 0),
                ]),
            list_of_voters: Vec::new(),
            voting_history: HashMap::new(),
         }
    }

    pub fn get_list_of_voters(&self) -> Vec<String> {
        return self.list_of_voters.clone()
    }

    pub fn insert_new_voter(&mut self, new_voter: String) {
        self.list_of_voters.push(new_voter);
    }

    pub fn is_voter_existing(&self, voter: String) -> bool {
        if self.list_of_voters.contains(&voter) {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_voting_by_name(&self, name: String) -> Vec<String> {
        self.voting_history.get(&name).unwrap().clone()
    }

    pub fn insert_new_voting(&mut self, new_voter: String, votes: Vec<String>) {
        self.voting_history.insert(new_voter, votes);
    }

    pub fn check_map_length(&self, input_map: HashMap<String, u64>) -> bool {
        if input_map.len() != 10 {
            panic!("There must be 10 distinct countries!");
        }
        else {
            return true
        }
    }

    pub fn check_input_list(&self, input_list: Vec<String>) -> bool {
        let mut input_list_copy = input_list.clone();
        input_list_copy.sort();
        input_list_copy.dedup();
        if input_list.len() != 10 || input_list_copy.len() != 10 {
            panic!("There must be 10 distinct countries!");
        }
        else {
            return true
        }
    }

    pub fn update_scoreboard(&mut self, input_map: HashMap<String, u64>) {
        let map_to_insert = input_map.clone();
        let keys_of_map_to_insert: Vec<String> = map_to_insert.into_keys().collect();
        for key_value in keys_of_map_to_insert {
            let key = key_value.clone();
            if self.scoreboard.contains_key(&key_value) {
                *self.scoreboard.get_mut(&key_value).unwrap() += input_map.get(&key_value).unwrap();    
            }
            else {
                self.scoreboard.insert(key, *input_map.get(&key_value).unwrap());
            }
        }

    }

    pub fn update_scoreboard_with_list(&mut self, input_list: Vec<String>, voter: String) {
        let list_to_insert = input_list.clone();
        let points: [u64; 10] = [12, 10, 8, 7, 6, 5, 4, 3, 2, 1];
        for (idx, country) in enumerate(list_to_insert) {
            if self.scoreboard.contains_key(&country) {
                *self.scoreboard.get_mut(&country).unwrap() += points[idx]; 
            }
            else {
                self.scoreboard.insert(country, points[idx]);
            }
        }
        self.voting_history.insert(voter, input_list);
    }

    pub fn get_scoreboard(&self) -> HashMap<String, u64> {
        return self.scoreboard.clone();
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::VMContextBuilder;
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn new() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        let contract = EscVoting::new();
        assert_eq!(10, contract.scoreboard.len());
    }

    #[test]
    #[should_panic]
    fn check_map_length_panic() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let contract = EscVoting::new();
        let map_to_insert = HashMap::from([
            (String::from("Ukraine"), 12),
            (String::from("Ukraine"), 10),
            (String::from("Ukraine"), 8),
            (String::from("Greece"), 7),
            (String::from("Cyprus"), 6),
            (String::from("Romania"), 5),
            (String::from("Croatia"), 4),
            (String::from("Portugal"), 3),
            (String::from("Sweden"), 2),
            (String::from("Switzerland"), 1),
            ]);
            
        contract.check_map_length(map_to_insert);
    }

    #[test]
    fn check_map_length_return_value() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let contract = EscVoting::new();
        let map_to_insert = HashMap::from([
            (String::from("Ukraine"), 12),
            (String::from("Denmark"), 10),
            (String::from("The Netherlands"), 8),
            (String::from("Greece"), 7),
            (String::from("Cyprus"), 6),
            (String::from("Romania"), 5),
            (String::from("Croatia"), 4),
            (String::from("Portugal"), 3),
            (String::from("Sweden"), 2),
            (String::from("Switzerland"), 1),
            ]);
        assert_eq!(contract.check_map_length(map_to_insert), true)
    }

    #[test]
    pub fn update_scoreboard() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let map_to_insert = HashMap::from([
            (String::from("Ukraine"), 12),
            (String::from("Denmark"), 10),
            (String::from("The Netherlands"), 8),
            (String::from("Greece"), 7),
            (String::from("Cyprus"), 6),
            (String::from("Romania"), 5),
            (String::from("Croatia"), 4),
            (String::from("Portugal"), 3),
            (String::from("Sweden"), 2),
            (String::from("Switzerland"), 1),
            ]);
        
        contract.update_scoreboard(map_to_insert.clone());
        assert_eq!(contract.scoreboard, map_to_insert)
    }

    #[test]
    #[should_panic]
    pub fn check_input_list() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let contract = EscVoting::new();
        let input_list = vec![String::from("Ukraine"), String::from("Ukraine"), String::from("Ukraine"),
                                    String::from("Ukraine"), String::from("Ukraine"), String::from("Ukraine"), 
                                    String::from("Ukraine"), String::from("Ukraine"), String::from("Ukraine"), String::from("Ukraine")];
        contract.check_input_list(input_list);
    }

    #[test]
    pub fn update_scoreboard_with_list() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let input_list = vec![String::from("Ukraine"), String::from("Denmark"), String::from("The Netherlands"),
                                    String::from("Greece"), String::from("Cyprus"), String::from("Romania"), 
                                    String::from("Croatia"), String::from("Portugal"), String::from("Sweden"), String::from("Switzerland")];
        
        let map_to_compare: HashMap<String, u64> = HashMap::from([
            (String::from("Ukraine"), 12),
            (String::from("Denmark"), 10),
            (String::from("The Netherlands"), 8),
            (String::from("Greece"), 7),
            (String::from("Cyprus"), 6),
            (String::from("Romania"), 5),
            (String::from("Croatia"), 4),
            (String::from("Portugal"), 3),
            (String::from("Sweden"), 2),
            (String::from("Switzerland"), 1),
            ]);
        contract.update_scoreboard_with_list(input_list, String::from("M"));
        assert_eq!(contract.scoreboard, map_to_compare)
    }

    #[test]
    pub fn  get_and_insert_voting_by_name() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        let input_list = vec![String::from("Ukraine"), String::from("Denmark"), String::from("The Netherlands"),
                                    String::from("Greece"), String::from("Cyprus"), String::from("Romania"), 
                                    String::from("Croatia"), String::from("Portugal"), String::from("Sweden"), String::from("Switzerland")];

        contract.insert_new_voting(String::from("M"), input_list.clone());

        assert_eq!(contract.get_voting_by_name(String::from("M")), input_list);


    }

    #[test]
    pub fn is_voter_existing() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = EscVoting::new();
        assert_eq!(contract.is_voter_existing(String::from("user.testnet")), false);
    }
}
