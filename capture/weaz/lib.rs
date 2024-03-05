#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod weaz {

    use ink::prelude::string::String;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Weaz {
        value: bool,
        digital_asset: String,
    }

    impl Weaz {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: true,
                digital_asset: "My great Link to a wonderful fire picture".into(),
            }
        }

        #[ink(message)]
        pub fn get_digital_asset(&self) -> String {
            self.digital_asset.clone()
        }

        #[ink(message)]
        pub fn change_digital_asset(&mut self, _asset: String) {
            self.digital_asset = _asset;
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
