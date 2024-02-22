#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod daffodils {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Daffodils {
        seed: String,
        area: String,
        proposed_seed: String,
        value: bool,
    }

    impl Daffodils {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                seed: String::from("Daffodils"),
                area: String::from("DistrictA, "),
                proposed_seed: String::from(""),
                value: true,
            }
        }

        #[ink(message)]
        pub fn add_seed(&mut self, _seed: String) {
            self.seed.push_str(&_seed);
        }

        #[ink(message)]
        pub fn add_area(&mut self) {
            self.area.push_str("DistrictX, ");
        }

        #[ink(message)]
        pub fn propose_seed(&mut self) {
            self.proposed_seed = String::from("Carlos");
        }

        #[ink(message)]
        pub fn get_propose_seed(&self) -> String {
            self.proposed_seed.clone()
        }

        #[ink(message)]
        pub fn get_seed(&self) -> String {
            self.seed.clone()
        }

        #[ink(message)]
        pub fn get_area(&mut self) -> String {
            self.area.clone()
        }

        #[ink(message)]
        pub fn fund(&self) {}

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
