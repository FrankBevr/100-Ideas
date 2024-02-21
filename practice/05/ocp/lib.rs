#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ocp {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Ocp {
        value: bool,
        input: String,
        offer: String,
    }

    impl Ocp {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: true,
                input: String::from("I want 5 rocks"),
                offer: String::from("No offer mi amigo")
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
        pub fn make_offers(&mut self) {
            self.offer = String::from("mining,5euro;packaging,3euro") 
        }

        #[ink(message)]
        pub fn get_best(&mut self) -> String {
            self.offer.clone()
        }
    }
}
