use std::cmp::max;

use solana_pubkey::Pubkey;

use crate::protos;

impl From<&protos::MessageHeader> for solana_message::MessageHeader {
    fn from(value: &protos::MessageHeader) -> Self {
        solana_message::MessageHeader {
            num_required_signatures: max(1, value.num_required_signatures as u8),
            num_readonly_signed_accounts: value.num_readonly_signed_accounts as u8,
            num_readonly_unsigned_accounts: value.num_readonly_unsigned_accounts as u8,
        }
    }
}

impl From<&protos::CompiledInstruction>
    for solana_message::compiled_instruction::CompiledInstruction
{
    fn from(value: &protos::CompiledInstruction) -> Self {
        solana_message::compiled_instruction::CompiledInstruction {
            program_id_index: value.program_id_index as u8,
            accounts: value.accounts.iter().map(|idx| *idx as u8).collect(),
            data: value.data.clone(),
        }
    }
}

impl From<&protos::MessageAddressTableLookup> for solana_message::v0::MessageAddressTableLookup {
    fn from(value: &protos::MessageAddressTableLookup) -> Self {
        solana_message::v0::MessageAddressTableLookup {
            account_key: Pubkey::new_from_array(value.account_key.clone().try_into().unwrap()),
            writable_indexes: value
                .writable_indexes
                .iter()
                .map(|idx| *idx as u8)
                .collect(),
            readonly_indexes: value
                .readonly_indexes
                .iter()
                .map(|idx| *idx as u8)
                .collect(),
        }
    }
}
