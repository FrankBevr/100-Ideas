#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod state_creator_04 {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct StateCreator04 {
        value: bool,
        citizien: AccountId,
        festival: String,
    }

    impl StateCreator04 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: true,
                citizien: AccountId::from([0x0; 32]),
                festival: String::from(""),
            }
        }

        #[ink(message)]
        pub fn become_citizen(&mut self){
            self.citizien = self.env().caller();
        }

        #[ink(message)]
        pub fn get_citizen(&self) -> AccountId{
            self.citizien
        }

        #[ink(message)]
        pub fn do_festival(&mut self){
            self.festival = "20240324,Herbert,50".into();
        }

        #[ink(message)]
        pub fn get_festival(&mut self) -> String {
            self.festival.clone()
        }

        #[ink(message)]
        pub fn buy_land(& self){}

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
