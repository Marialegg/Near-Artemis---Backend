//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! [get it's current value][get_num] or [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Balance};
use near_sdk::collections::{ UnorderedMap};
//use near_sdk::json_types::{U128};
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use near_sdk::json_types::ValidAccountId;
//use near_sdk::env::is_valid_account_id;


near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileObject {
    user_id: AccountId,
    purchased_courses: Option<Vec<i128>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesObject {
	name: String,
    img: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesJson {
    id: i128,
	name: String,
    img: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TemplateObject {
	title: String,
    description: String,
    content: String,
    tipo: i8, // 1 Video, 2 Text
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CoursesObject {
    id: i128,
    creator_id: AccountId,
    title: String,
    categories: CategoriesJson,
    short_description: String,
    long_description: String,
    img: String,
    content: Vec<TemplateObject>,
    price: Balance,
    inscriptions: Option<Vec<AccountId>>,
    reviews: Option<Vec<Review>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Review {
    user_id: AccountId,
    review: String,
    critics: i8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CoursesInstructor {
    id: i128,
    creator_id: AccountId,
    title: String,
    categories: CategoriesJson,
    short_description: String,
    long_description: String,
    img: String,
    content: Vec<TemplateObject>,
    price: Balance,
    inscriptions: Option<Vec<AccountId>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketView {
    id: i128,
    creator_id: AccountId,
    title: String,
    categories: CategoriesJson,
    short_description: String,
    img: String,
    price: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    vault_id: AccountId,
    profiles: UnorderedMap<AccountId, ProfileObject>,
    id_categories: i128,
    categories: Vec<CategoriesJson>,
    id_courses: i128,
    courses: UnorderedMap<i128, CoursesObject>,
    administrators: Vec<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, vault_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            vault_id,
        )
    }

    #[init]
    pub fn new(_owner_id: ValidAccountId, vault_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            vault_id: vault_id.to_string(),
            profiles: UnorderedMap::new(b"s".to_vec()),
            id_categories: 0,
            categories: Vec::new(),
            id_courses: 0,
            courses: UnorderedMap::new(b"s".to_vec()),
            administrators: vec![
                                    "e-learning.testnet".to_string(),
                                    "juanochando.testnet".to_string(),
                                ],
        }
    }

    pub fn set_admin(&mut self, user_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        let valid = self.administrators.iter().find(|&x| x == &user_id);
        if valid.is_some() {
            env::panic(b"the user is already in the list of administrators");
        }
        self.administrators.push(user_id);
    }

    pub fn delete_admin(&mut self, user_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        let index = self.administrators.iter().position(|x| x == &user_id.to_string()).expect("the user is not in the list of administrators");
        self.administrators.remove(index);
    }

    pub fn set_profile(&mut self, 
        purchased_courses: Option<Vec<i128>>,
    ) -> ProfileObject {
        let profile = self.profiles.get(&env::signer_account_id());
        if profile.is_some() {
            env::panic(b"profile already exists");
        }
        
        let data = ProfileObject {
            user_id: env::signer_account_id().to_string(),
            purchased_courses: purchased_courses,
        };

        self.profiles.insert(&env::signer_account_id(), &data);
        env::log(b"profile Created");
        data
    }

    pub fn put_profile(&mut self, 
        purchased_courses: Option<Vec<i128>>,
    ) -> ProfileObject {
        let return_data = ProfileObject {
            user_id: env::signer_account_id().to_string(),
            purchased_courses: purchased_courses.clone(),
        };
        let mut profile = self.profiles.get(&env::signer_account_id()).expect("Profile does not exist");
        profile.user_id = env::signer_account_id().to_string();
        profile.purchased_courses = profile.purchased_courses;

        self.profiles.insert(&env::signer_account_id(), &profile);

        env::log(b"profile Update");

        return_data
    }


    pub fn get_profile(&self, user_id: AccountId) -> ProfileObject {
        let profile = self.profiles.get(&user_id).expect("Profile does not exist");

        ProfileObject {
            user_id: profile.user_id,
            purchased_courses: profile.purchased_courses,
        }
	}

    pub fn set_category(&mut self, name: String, img: String) -> CategoriesJson {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        self.id_categories += 1;
        let data = CategoriesJson {
            id: self.id_categories,
            name: name.to_string(),
            img: img.to_string(),
        };
        
        self.categories.push(data.clone());
        env::log(b"category Created");
        
        data
    }

    pub fn put_category(&mut self, category_id: i128, name: String, img: String) -> CategoriesJson {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only admins can edit categories");
        let index = self.categories.iter().position(|x| x.id == category_id).expect("Category does not exist");
        self.categories[index].name = name.to_string();
        self.categories[index].img = img.to_string();

        env::log(b"Category Update");

        CategoriesJson {
            id: category_id,
            name: name.to_string(),
            img: img.to_string(),
        }
    }

    pub fn get_category(&self, category_id: Option<i128>) -> Vec<CategoriesJson> {
        let mut categories = self.categories.clone();

        if category_id.is_some() {
            categories = self.categories.iter().filter(|x| x.id == category_id.unwrap()).map(|x| CategoriesJson {
                id: x.id,
                name: x.name.to_string(),
                img: x.img.to_string(),
            }).collect();
        }
        categories
    }

    pub fn delete_category(&mut self, category_id: i128) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only admins can edit categories");
        let index = self.categories.iter().position(|x| x.id == category_id).expect("Category does not exist");
        self.categories.remove(index);

        env::log(b"Category deleted");
    }
    
    pub fn publish_course(&mut self, 
        title: String,
        categories: CategoriesJson,
        short_description: String,
        long_description: String,
        img: String,
        content: Vec<TemplateObject>,
        price: Balance,
    ) -> CoursesObject {
        
        self.id_courses += 1;
        let data = CoursesObject {
            id: self.id_courses,
            creator_id: env::signer_account_id().to_string(),
            title: title.to_string(),
            categories: categories,
            short_description: short_description.to_string(),
            long_description: long_description.to_string(),
            img: img.to_string(),
            content: content,
            price: price,
            inscriptions: None,
            reviews: None,
        };

        self.courses.insert(&self.id_courses, &data);
        env::log(b"published course");
        data
    }

    pub fn get_cources_intructor(&self, user_id: Option<String>) -> Vec<CoursesObject> {
        if user_id.is_some() {
            self.courses.iter().filter(|(_k, x)| x.creator_id == user_id.clone().unwrap().to_string()).map(|(_k, x)| CoursesObject {
                id: x.id,
                creator_id: x.creator_id.to_string(),
                title: x.title.to_string(),
                categories: x.categories.clone(),
                short_description: x.short_description.to_string(),
                long_description: x.long_description.to_string(),
                img: x.img.to_string(),
                content: x.content.clone(),
                price: x.price,
                inscriptions: x.inscriptions.clone(),
                reviews: x.reviews.clone(),
            }).collect()
        } else {
            env::panic(b"Not user");
        }
    }

    pub fn get_market_cources(&self) -> Vec<MarketView> {
        self.courses.iter().map(|(_k, x)| MarketView {
            id: x.id,
            creator_id: x.creator_id.to_string(),
            title: x.title.to_string(),
            categories: x.categories.clone(),
            short_description: x.short_description.to_string(),
            img: x.img.to_string(),
            price: x.price,
        }).collect()
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}