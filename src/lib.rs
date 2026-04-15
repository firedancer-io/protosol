pub mod protos {
    include!("generated/org.solana.sealevel.v1.rs");
}

/// Pre-compiled FileDescriptorSet for all protobuf definitions.
///
/// This binary blob can be loaded by reflection libraries (e.g. prost-reflect's
/// `DescriptorPool::decode`) to inspect message schemas at runtime.
pub const FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!("generated/file_descriptor_set.bin");

#[allow(
    unused_imports,
    dead_code,
    clippy::default_trait_access,
    clippy::derivable_impls,
    clippy::needless_lifetimes,
    clippy::used_underscore_binding,
    clippy::extra_unused_lifetimes,
    clippy::missing_safety_doc,
    unsafe_code
)]
pub mod context_generated {
    include!("generated/context_generated.rs");
    pub use self::org::solana::sealevel::v_2::*;
}

#[allow(
    unused_imports,
    dead_code,
    clippy::default_trait_access,
    clippy::derivable_impls,
    clippy::needless_lifetimes,
    clippy::used_underscore_binding,
    clippy::extra_unused_lifetimes,
    clippy::missing_safety_doc,
    unsafe_code
)]
pub mod elf_generated {
    include!("generated/elf_generated.rs");
    pub use self::org::solana::sealevel::v_2::*;
}

#[allow(
    unused_imports,
    dead_code,
    clippy::default_trait_access,
    clippy::derivable_impls,
    clippy::needless_lifetimes,
    clippy::used_underscore_binding,
    clippy::extra_unused_lifetimes,
    clippy::missing_safety_doc,
    unsafe_code
)]
pub mod metadata_generated {
    include!("generated/metadata_generated.rs");
    pub use self::org::solana::sealevel::v_2::*;
}

#[cfg(feature = "solana-types")]
pub mod convert;

#[cfg(test)]
mod tests {
    use crate::protos;

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
}
