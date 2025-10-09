pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/org.solana.sealevel.v1.rs"));
}

#[cfg(test)]
mod tests {
    use crate::protos;

    #[test]
    fn can_create_txn() {
        let txn = protos::SanitizedTransaction {
            ..Default::default()
        };
        assert_eq!(txn, protos::SanitizedTransaction { ..Default::default() }); 
    }
}
