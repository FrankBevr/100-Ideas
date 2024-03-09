#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod animalo {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Animalo {
        value: bool,
        animal: String,
        rewarder: AccountId,
    }

    impl Animalo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                animal: "Berry".into(),
                rewarder: AccountId::from([0xff; 32]),
            }
        }

        #[ink(message)]
        pub fn get_current_rewarder(&self) -> AccountId {
            self.rewarder
        }

        #[ink(message)]
        pub fn pay(&self) {}

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
