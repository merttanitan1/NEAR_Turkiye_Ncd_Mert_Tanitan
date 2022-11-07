use std::ptr::null;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::ser::SerializeTupleVariant;
use near_sdk::{log, env, near_bindgen};



#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u32,
    
}   
    
#[near_bindgen]
impl Counter {
    // ----------------x1-------------------- //
    // Public read-only method: Returns the counter value.
    pub fn get_num(&self) -> u32 {
        return self.val;
    }

    // Public method: Increment the counter.
    pub fn increment(&mut self) {
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    // Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
    }

    // Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }
    // Public method: Multiplies the number you typed by the first number
    pub fn multiple(&mut self, number: u32)   {
        self.val *= number;
        env::log_str(&format!("this is new number {}", (self.val)));
    }

    pub fn divide(&mut self) {
        if  self.val % 2 == 0 {
            log!("Your number {} is even", self.val);
        }
        else {
            log!("Your number {} is odd", self.val);
        }
    }
    pub fn change_to_even(&mut self) {
        assert!(self.val % 2 != 0, "Number is already even");
        self.val +=1;
    }
    pub fn change_to_odd(&mut self) {
        assert!(self.val % 2 == 0, "Number is already odd");
        self.val +=1;
    }
}