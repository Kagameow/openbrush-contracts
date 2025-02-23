// #![cfg_attr(test, allow(unused_imports))]

use crate::traits::{Erc1155Error, Id};
#[cfg(not(test))]
use crate::stub::{Erc1155Receiver};
#[cfg(not(test))]
use ink_env::{
    call::{FromAccountId},
    Error as Env_error,
};
#[cfg(not(test))]
use ink_lang::ForwardCallMut;
#[cfg(not(test))]
use ink_prelude::string::String;
use ink_prelude::{vec::Vec, vec};
use ink_storage::{
    collections::HashMap as StorageHashMap,
};
use brush::{
    traits::{InkStorage, AccountId, Balance},
    define_getters,
};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Erc1155MetadataURIStorage: InkStorage {
    define_getters!(_uri, _uri_mut, Option<String>);
}

pub trait Erc1155MetadataURI: Erc1155MetadataURIStorage {
    fn new(uri: Option<String>) -> Self {
        let mut instance = Self::_empty();
        *instance._uri_mut() = uri;
        instance
    }

    fn uri(&self, _id: Id) -> Option<String> {
        self._uri().clone()
    }
}

pub trait Erc1155Storage: InkStorage {
    define_getters!(_balances, _balances_mut, StorageHashMap<(Id, AccountId), Balance>);

    define_getters!(_operator_approval, _operator_approval_mut, StorageHashMap<(AccountId, AccountId), bool>);
}

pub trait Erc1155: Erc1155Storage {
    fn emit_transfer_single_event(&self,
                                  _operator: AccountId, _from: AccountId,
                                  _to: AccountId, _id: Id, _amount: Balance);

    fn emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool);

    fn emit_transfer_batch_event(&self,
                                 _operator: AccountId, _from: AccountId,
                                 _to: AccountId, _ids: Vec<Id>, _amounts: Vec<Balance>);

    fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
        self._balance_of_or_zero(_account, _id)
    }

    fn balance_of_batch(
        &self,
        _accounts: Vec<AccountId>,
        _ids: Vec<Id>,
    ) -> Vec<Balance> {
        assert_eq!(_accounts.len(), _ids.len(), "{}", Erc1155Error::InputLengthMismatch.as_ref());

        let values: Vec<Balance> = _accounts
            .iter()
            .zip(_ids.iter())
            .map(|(account, id)| self._balance_of_or_zero(account.clone(), id.clone()))
            .collect();
        values
    }

    fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) {
        let caller = Self::env().caller();
        assert_ne!(caller, _operator, "{}", Erc1155Error::SelfApproval.as_ref());
        *self
            ._operator_approval_mut()
            .entry((Self::env().caller(), _operator))
            .or_insert(false) = _approved;

        self.emit_approval_for_all_event(caller, _operator, _approved);
    }

    fn is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self._is_approved_for_all(_account, _operator)
    }

    fn safe_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) {
        self._transfer_guard(_from, _to);
        self._before_token_transfer(&vec![_id]);
        self._transfer_from(_from, _to, _id, _amount);

        self._do_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _id,
            _amount,
            _data
        );

        self.emit_transfer_single_event(
            Self::env().caller(), _from, _to, _id, _amount);
    }

    fn safe_batch_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {
        assert_eq!(_ids.len(), _amounts.len(), "{}", Erc1155Error::InputLengthMismatch.as_ref());
        self._transfer_guard(_from, _to);
        self._before_token_transfer(&_ids);

        for (id, value) in _ids.iter().zip(_amounts.iter()) {
            self._transfer_from(_from, _to, id.clone(), value.clone());
        }

        self._do_batch_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _ids.clone(),
            _amounts.clone(),
            _data,
        );

        self.emit_transfer_batch_event(
            Self::env().caller(), _from, _to, _ids, _amounts);
    }


    fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
        let operator = Self::env().caller();

        assert_ne!(to, ZERO_ADDRESS.into(), "{}", Erc1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._increase_receiver_balance(to, id, amount);

        self._do_safe_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            id,
            amount,
            Vec::new(),
        );

        self.emit_transfer_single_event(
            operator, ZERO_ADDRESS.into(), to, id, amount);
    }

    fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        assert_ne!(from, ZERO_ADDRESS.into(), "{}", Erc1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._decrease_sender_balance(from, id, amount);

        self.emit_transfer_single_event(
            Self::env().caller(), from, ZERO_ADDRESS.into(), id, amount);
    }

    // Internal functions

    #[inline]
    fn _transfer_guard(&self, from: AccountId, to: AccountId) {
        assert_ne!(to, ZERO_ADDRESS.into(), "{}", Erc1155Error::TransferToZeroAddress.as_ref());

        let operator = Self::env().caller();

        if (from != operator) && (!self._is_approved_for_all(from, operator)) {
            panic!("{}", Erc1155Error::ApproveRequired.as_ref());
        }
    }

    #[inline]
    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) {
        self._decrease_sender_balance(from, id, amount);
        self._increase_receiver_balance(to, id, amount);
    }

    #[inline]
    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self._balances().get(&(id, owner)).cloned().unwrap_or(0)
    }
    #[inline]
    fn _is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self._operator_approval().get(&(_account, _operator)).cloned().unwrap_or(false)
    }

    #[inline]
    fn _increase_receiver_balance(
        &mut self,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) {
        let to_balance = self._balances_mut().entry((id, to)).or_insert(0);
        match to_balance.checked_add(amount) {
            Some(new_to_balance) => *to_balance = new_to_balance,
            _ => panic!("{}", Erc1155Error::MaxBalance.as_ref()),
        }
    }

    #[inline]
    fn _decrease_sender_balance(
        &mut self,
        from: AccountId,
        id: Id,
        amount: Balance,
    ) {
        match self
            ._balances()
            .get(&(id, from))
            .map(|old_from_balance| old_from_balance.checked_sub(amount))
        {
            Some(Some(new_from_balance)) => self._balances_mut().insert((id, from), new_from_balance),
            _ => panic!("{}", Erc1155Error::InsufficientBalance.as_ref()),
        };
    }

    #[inline]
    fn _before_token_transfer(&self, _ids: &Vec<Id>) {}

    #[cfg(test)]
    #[inline]
    fn _do_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) {}

    #[cfg(not(test))]
    #[inline]
    fn _do_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) {
        let mut receiver : Erc1155Receiver = FromAccountId::from_account_id(_to);
        match receiver.call_mut().on_erc1155_received(_operator, _from, _id, _amount, _data).fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                _ => panic!("{}", Erc1155Error::CallFailed.as_ref()),
            },
            Err(e) => match e {
                Env_error::NotCallable => (),
                _ => panic!("{}", Erc1155Error::CallFailed.as_ref()),
            },
        }
    }

    #[cfg(test)]
    #[inline]
    fn _do_batch_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {}

    #[cfg(not(test))]
    #[inline]
    fn _do_batch_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {
        let mut receiver : Erc1155Receiver = FromAccountId::from_account_id(_to);
        match receiver.call_mut().on_erc1155_batch_received(_operator, _from, _ids, _amounts, _data).fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                _ => panic!("{}", Erc1155Error::CallFailed.as_ref()),
            },
            Err(e) => match e {
                Env_error::NotCallable => (),
                _ => panic!("{}", Erc1155Error::CallFailed.as_ref()),
            },
        }
    }
}
