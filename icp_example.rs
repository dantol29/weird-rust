use ic_cdk::*;
use candid::{Principal};
use std::cell::RefCell;
use std::string::String;
use std::collections::HashMap;

type UserData = HashMap<Principal, String>; // create a map with principal_id: username key-value pair

thread_local! { // create thread-safe variable scope
  static PROFILE: RefCell<UserData> = RefCall::default(); // create global(static) variable to store information on-chain
}

// retrieve an username with a principal_id
#[ic_cdk_macros::query]
fn get_self() -> String {
  let id = ic_cdk::api::caller() // get principal id of the caller
  PROFILE.width(|profile| {
    profile.borrow() // profile.borrow() - returns immutable RefCell<UserData>
           .get(&id) // .get(&id) - returns Option<&V>, where V is the value stored in the hash table
           .cloned() // .cloned() - transforms an Option<&T> into an Option<T>, where T is the cloned value of the reference
           .unwrap_or_else(|| "Default Name".to_string()) // .unwrap_or_else() - allows to specify what will be executed when the `Option` is `None` or the `Result` is `Err`   
  }
}

// add a new user (principal_id, name)
#[ic_cdk_macros::update]
fn set(name: String) {
  let id = ic_cdk::api()::caller() // get principal id of the caller
  PROFILE.with(|profile|{
    profile.borrow_mut()     // profile.borrow_mut() - returns mutable RefCell<UserData>
           .insert(id, name) // .insert() - add a new key-value pair to the HashMap
  })
}


Candid file: 
  
  service : {
    "get" : () -> (text) query;
    "set" : (text) -> ();
};
