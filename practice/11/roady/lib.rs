#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod roady {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Roady {
        value: bool,
        user: AccountId,
        ai: String,
    }

    impl Roady {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                user: AccountId::from([0xff; 32]),
                ai: String::from("no great link to wonderful ai"),
            }
        }

        #[ink(message)]
        pub fn get_ai(&self) -> String {
            self.ai.clone()
        }

        #[ink(message)]
        pub fn generate(&mut self) {
            self.ai = String::from("link to great ai");
        }

        #[ink(message)]
        pub fn who_is_the_owner(&self) -> AccountId {
            self.user
        }

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
