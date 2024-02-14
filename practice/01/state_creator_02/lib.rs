#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod state_creator_02 {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct StateCreator02 {
        citizen: AccountId,
        festival: String,
    }

    impl StateCreator02 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                citizen: AccountId::from([0x0; 32]),
                festival: String::from(""),
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {}

        #[ink(message)]
        pub fn get(&self) {}
    }
}
