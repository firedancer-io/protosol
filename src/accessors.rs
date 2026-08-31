use crate::protos::{acct_state::DataRepr, AcctState};

impl AcctState {
    /// Account data, or an empty slice when this account carries a hash instead
    /// of its contents.
    pub fn data(&self) -> &[u8] {
        match &self.data_repr {
            Some(DataRepr::Data(data)) => data.as_slice(),
            _ => &[],
        }
    }

    /// Mutable account data, switching the representation to inline contents if
    /// it currently holds a hash.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        if !matches!(self.data_repr, Some(DataRepr::Data(_))) {
            self.data_repr = Some(DataRepr::Data(Vec::new()));
        }
        match &mut self.data_repr {
            Some(DataRepr::Data(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data_repr = Some(DataRepr::Data(data));
    }

    /// The account data hash, or 0 when this account carries inline contents.
    pub fn data_hash(&self) -> u64 {
        match self.data_repr {
            Some(DataRepr::DataHash(hash)) => hash,
            _ => 0,
        }
    }

    pub fn set_data_hash(&mut self, hash: u64) {
        self.data_repr = Some(DataRepr::DataHash(hash));
    }
}
