#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod stylo {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Stylo {
        value: bool,
        style: String,
        stylist: AccountId,
    }

    impl Stylo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                style: String::from("{style:Garbani Hat 400, color: blue, accesoir: perls}"),
                stylist: AccountId::from([0xff; 32]),
            }
        }

        #[ink(message)]
        pub fn get_style(&self) -> String {
            self.style.clone()
        }

        #[ink(message)]
        pub fn set_style(&mut self, _another_style: String) {
            self.style = String::from(_another_style);
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
