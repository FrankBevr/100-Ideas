#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod citor {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Citor {
        value: bool,
        area: String,
        gas_data: String,
    }

    impl Citor {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                area: String::from("Area52,"),
                gas_data: String::from("5,Good; "),
            }
        }

        #[ink(message)]
        pub fn get_area(&self) -> String {
            self.area.clone()
        }

        #[ink(message)]
        pub fn add_area(&mut self) {
            self.area.push_str(" Area 53,");
        }

        #[ink(message)]
        pub fn get_gas_data(&self) -> String {
            self.gas_data.clone()
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
