mod account;
mod block;
mod context;
mod txn;

pub type KeyedAccountSharedData = (solana_pubkey::Pubkey, solana_account::AccountSharedData);
