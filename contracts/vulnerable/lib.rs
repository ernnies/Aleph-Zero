#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod price {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Price {
        total_supply: u32,
        price: u32,
        owner: AccountId,
        balances: Mapping<AccountId, u32>,
    }

    impl Price {
        #[ink(constructor)]
        pub fn new(supply: u32, price: u32) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &supply);

            Self {
                total_supply: supply,
                price,
                owner: caller,
                balances,
            }
        }

        #[ink(message)]
        pub fn set_price(&mut self, price: u32) {
            self.price = price;
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) {
            self.owner = new_owner;
        }
    }
}
