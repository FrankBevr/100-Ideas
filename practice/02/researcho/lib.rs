#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod researcho {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Researcho {
        value: bool,
        proposal: String,
        member: AccountId,
    }

    impl Researcho {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: true, 
                proposal: String::from(""),
                member: AccountId::from([0x0;32])
            }
        }

        #[ink(message)]
        pub fn get_proposal(& self) -> String {
            self.proposal.clone()
        }

        #[ink(message)]
        pub fn become_member(&mut self){
            self.member = self.env().caller();
        }

        #[ink(message)]
        pub fn get_member(&mut self) -> AccountId{
            self.member
        }

        #[ink(message)]
        pub fn send_proposal(&self) {}

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
