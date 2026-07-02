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
    fn exports_crate_version() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
