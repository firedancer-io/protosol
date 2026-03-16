use solana_account::{Account, AccountSharedData, ReadableAccount, WritableAccount};
use solana_pubkey::Pubkey;

use super::KeyedAccountSharedData;
use crate::protos::AcctState;

impl From<&AcctState> for AccountSharedData {
    fn from(input: &AcctState) -> Self {
        let mut account_data = AccountSharedData::default();
        account_data.set_lamports(input.lamports);
        account_data.set_data_from_slice(input.data.as_slice());
        account_data.set_owner(Pubkey::new_from_array(
            input.owner.clone().try_into().unwrap(),
        ));
        account_data.set_executable(input.executable);
        account_data.set_rent_epoch(u64::MAX);
        account_data
    }
}

impl From<AcctState> for (Pubkey, Account) {
    fn from(input: AcctState) -> Self {
        let pubkey = Pubkey::new_from_array(input.address.try_into().unwrap());
        let owner = Pubkey::new_from_array(input.owner.try_into().unwrap());
        (
            pubkey,
            Account {
                lamports: input.lamports,
                data: input.data,
                owner,
                executable: input.executable,
                rent_epoch: u64::MAX,
            },
        )
    }
}

impl From<KeyedAccountSharedData> for AcctState {
    fn from(value: KeyedAccountSharedData) -> AcctState {
        AcctState {
            address: value.0.to_bytes().to_vec(),
            lamports: value.1.lamports(),
            data: value.1.data().to_vec(),
            executable: value.1.executable(),
            owner: value.1.owner().to_bytes().to_vec(),
        }
    }
}
