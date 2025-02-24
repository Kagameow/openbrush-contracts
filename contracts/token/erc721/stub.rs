// It exports the stub implementation of all Erc721 traits.
// ink! will generate a wrapper around all methods of each trait and it will allow creating wrapped
// struct around contracts address(::ink_env::call::FromAccountId::from_account_id).
pub use self::erc721::{Erc721};
pub use self::erc721receiver::{Erc721Receiver};

#[ink_lang::contract(compile_as_dependency = true)]
mod erc721 {
    use ink_prelude::{ string::String, vec::Vec };
    use crate::traits::{ Id };

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc721 {}

    impl Erc721 {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc721")]
    impl Erc721 {
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            unimplemented!()
        }

        #[ink(message)]
        pub fn owner_of(&self, id: Id) -> Option<AccountId> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn get_approved(&self, id: Id) -> Option<AccountId> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            unimplemented!()
        }

        #[ink(message)]
        pub fn set_approval_for_all(&mut self, to: AccountId, approved: bool) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, id: Id) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
        ) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            data: Vec<u8>,
        ) {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc721Metadata")]
    impl Erc721 {
        #[ink(message)]
        pub fn name(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn symbol(&self) -> Option<String> {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc721Mint")]
    impl Erc721 {
        #[ink(message)]
        pub fn mint(&mut self, id: Id) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn(&mut self, id: Id) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod erc721receiver {
    use ink_prelude::{ vec::Vec };
    use crate::traits::{ Id, Erc721ReceiverError };

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc721Receiver {}

    impl Erc721Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc721Receiver")]
    impl Erc721Receiver {
        #[ink(message)]
        pub fn on_erc721_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Result<(), Erc721ReceiverError> {
            unimplemented!()
        }
    }
}