#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod state_creator {
    use ink::prelude::borrow::ToOwned;
    use ink::prelude::string::String;
    #[ink(storage)]
    pub struct StateCreator {
        citizen: AccountId,
        festival: String,
    }

    impl StateCreator {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                citizen: AccountId::from([0x0; 32]),
                festival: String::from(""),
            }
        }
        #[ink(message)]
        pub fn become_citizen(&mut self) {
            self.citizen = self.env().caller();
        }
        #[ink(message)]
        pub fn get_citizen(&mut self) -> AccountId {
            self.citizen
        }

        #[ink(message)]
        pub fn do_festival(&mut self) {
            self.festival = "01-05-2024, MetaFestival, 50Euro".into();
        }
        #[ink(message)]
        pub fn get_festival(&self) -> String {
            self.festival.to_owned()
        }
        #[ink(message, payable)]
        pub fn buy_land(&self) {}
    }
}
