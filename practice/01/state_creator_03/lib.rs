#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod state_creator_03 {
    use ink::prelude::string::String;
    use ink::prelude::borrow::ToOwned;

    #[ink(storage)]
    pub struct StateCreator03 {
        value: bool,
        citizen: AccountId,
        festival: String,
    }

    impl StateCreator03 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: false,
                citizen: AccountId::from([0x0; 32]),
                festival: String::from(""),
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn change_citizen(&mut self) {
            self.citizen = self.env().caller();
        }

        #[ink(message)]
        pub fn get_citizen(&self) -> AccountId {
            self.citizen
        }

        #[ink(message)]
        pub fn do_festival(&mut self) {
            self.festival = "20032024, Hello, 50".to_owned()
        }

        #[ink(message)]
        pub fn get_festival(&mut self) -> String {
            self.festival.clone()
        }


        #[ink(message)]
        pub fn buy_land(&self) {}
    }
}
