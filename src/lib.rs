pub mod protos {
    include!("generated/org.solana.sealevel.v1.rs");
}

/// Version of the `protosol` crate, sourced from Cargo package metadata.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Pre-compiled FileDescriptorSet for all protobuf definitions.
///
/// This binary blob can be loaded by reflection libraries (e.g. prost-reflect's
/// `DescriptorPool::decode`) to inspect message schemas at runtime.
pub const FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!("generated/file_descriptor_set.bin");

mod accessors;

#[cfg(feature = "solana-types")]
pub mod convert;

#[cfg(test)]
mod tests {
    use crate::{protos, VERSION};

    #[test]
    fn can_create_txn() {
        let txn = protos::SanitizedTransaction {
            ..Default::default()
        };
        assert_eq!(
            txn,
            protos::SanitizedTransaction {
                ..Default::default()
            }
        );
    }

    #[test]
    #[allow(deprecated)]
    fn version_and_config_roundtrip() {
        use prost::Message;
        // Pre-16.0.0 style message: version defaults to V0 and is_legacy carries
        // the legacy/V0 distinction.
        let legacy = protos::TransactionMessage {
            ..Default::default()
        };
        let decoded =
            protos::TransactionMessage::decode(legacy.encode_to_vec().as_slice()).unwrap();
        assert_eq!(decoded.version(), protos::TransactionVersion::V0);
        assert!(decoded.is_legacy && decoded.v1_config.is_none());

        let bare = protos::TransactionMessage {
            version: protos::TransactionVersion::V1 as i32,
            ..Default::default()
        };
        let decoded = protos::TransactionMessage::decode(bare.encode_to_vec().as_slice()).unwrap();
        assert_eq!(decoded.version(), protos::TransactionVersion::V1);
        assert!(decoded.v1_config.is_none());

        let v1 = protos::TransactionMessage {
            version: protos::TransactionVersion::V1 as i32,
            v1_config: Some(protos::TransactionConfig {
                priority_fee: Some(0),
                compute_unit_limit: None,
                loaded_accounts_data_size_limit: Some(0),
                heap_size: Some(32 * 1024),
            }),
            ..Default::default()
        };
        let decoded = protos::TransactionMessage::decode(v1.encode_to_vec().as_slice()).unwrap();
        assert_eq!(decoded, v1);
        assert_eq!(decoded.v1_config.as_ref().unwrap().priority_fee, Some(0));
        assert_eq!(decoded.v1_config.as_ref().unwrap().compute_unit_limit, None);
    }

    #[test]
    fn exports_crate_version() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
