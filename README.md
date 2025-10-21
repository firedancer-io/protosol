# Protosol

Protocol buffer definitions for the Solana Virtual Machine (SVM) testing harness in Agave. This crate provides Rust bindings for protobuf schemas used in fuzzing and testing the Solana blockchain's execution environment.

## Overview

Protosol defines the data structures used to capture, serialize, and replay Solana blockchain execution contexts for testing and fuzzing purposes. These protocol buffers represent the complete state and execution flow of Solana transactions, blocks, and virtual machine operations.

## What These Protocol Buffers Represent

### Core Components

- **Transaction Processing**: Complete transaction lifecycle from input to execution effects
- **Block Execution**: Block-level context including transactions, account states, and consensus data
- **Virtual Machine State**: SVM execution context, compute budgets, and program execution
- **Account Management**: Account states, ownership, and data storage
- **Consensus Context**: Slot information, epoch data, and leader schedules

### Key Message Types

#### Transaction Layer (`txn.proto`)
- `SanitizedTransaction`: Complete transaction with message and signatures
- `TransactionMessage`: Transaction payload with accounts and instructions
- `CompiledInstruction`: Individual program instructions with account references
- `MessageHeader`: Transaction metadata (signatures, read-only accounts)

#### Block Layer (`block.proto`)
- `BlockContext`: Input state for block execution (transactions, accounts, block hash queue)
- `BlockEffects`: Output state after block execution (bank hash, costs, leader schedule)
- `BlockFixture`: Complete test fixture combining input context and expected effects

#### Execution Context (`context.proto`)
- `AcctState`: Complete account state (address, lamports, data, owner)
- `FeatureSet`: Enabled Solana features for execution
- `SlotContext`: Slot-specific information (POH, parent hashes)
- `EpochContext`: Epoch-level data (vote accounts, feature sets)

#### Compute Budget (`pack.proto`)
- `PackComputeBudgetContext`: Input for compute budget testing
- `PackComputeBudgetEffects`: Output effects of compute budget execution

## Usage

### As a Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
protosol = "X.Y.Z"
```

### Basic Usage

```rust
use protosol::protos;

// Create a transaction fixture
let txn = protos::SanitizedTransaction {
    message: Some(protos::TransactionMessage {
        header: Some(protos::MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        }),
        account_keys: vec![/* account keys */],
        recent_blockhash: vec![/* blockhash */],
        instructions: vec![/* instructions */],
        address_table_lookups: vec![],
    }),
    signatures: vec![vec![/* signature bytes */]],
};

// Create account state
let account = protos::AcctState {
    address: vec![/* 32-byte address */],
    lamports: 1000000,
    data: vec![/* account data */],
    executable: false,
    owner: vec![/* 32-byte owner */],
};

// Create block context for testing
let block_ctx = protos::BlockContext {
    txns: vec![txn],
    acct_states: vec![account],
    blockhash_queue: vec![/* recent blockhashes */],
    slot_ctx: Some(protos::SlotContext {
        slot: 12345,
        poh: vec![/* POH hash */],
        parent_bank_hash: vec![/* parent hash */],
        parent_lthash: vec![/* parent LT hash */],
    }),
    epoch_ctx: Some(protos::EpochContext {
        epoch: 100,
        features: Some(protos::FeatureSet {
            features: vec![/* feature flags */],
        }),
        vote_accounts: vec![],
    }),
};
```

### Testing and Fuzzing

These protocol buffers are designed for:

1. **Test Fixture Generation**: Create reproducible test cases with complete blockchain state
2. **Fuzzing**: Generate random but valid blockchain execution contexts
3. **State Capture**: Record actual blockchain execution for later replay
4. **Regression Testing**: Compare execution results across different SVM implementations

### Integration with Agave

This crate is specifically designed for use with the anza-xyz/agave repository's testing infrastructure:

- **SVM Fuzzing**: Generate test cases for the Solana Virtual Machine
- **Execution Testing**: Validate transaction and block execution across different implementations
- **Performance Testing**: Measure execution costs and resource usage
- **Compatibility Testing**: Ensure different SVM implementations produce identical results

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Regenerating Protobuf Code

The protobuf code is automatically generated during build. To force regeneration:

```bash
cargo clean
cargo build
```

## File Structure

```
proto/
├── block.proto          # Block execution context and effects
├── context.proto        # Account states and execution context
├── txn.proto            # Transaction structures
├── pack.proto           # Compute budget testing
├── metadata.proto       # Test fixture metadata
└── *.options            # Nanopb configuration files
```

## License

Apache-2.0

## Repository

https://github.com/firedancer-io/protosol
