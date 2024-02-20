#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod diabetes {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Diabetes {
        value: bool,
        member: AccountId,
        leader: AccountId,
        achievments: String,
    }

    impl Diabetes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                value: true,
                member: AccountId::from([0xff;32]),
                leader: AccountId::from([0xff;32]),
                achievments: String::from("")
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
        pub fn rewarded(&self)  {}

        #[ink(message)]
        pub fn post_achievments(&mut self)  {
            self.achievments = String::from("Great Certifacte from the Fitness Studio");
        }

        #[ink(message)]
        pub fn get_achievments(&self) -> String  {
            self.achievments.clone()
        }

        #[ink(message)]
        pub fn get_leader(&self) -> AccountId  {
            self.leader
        }

        #[ink(message)]
        pub fn fund(&self)  {}

    }
}
