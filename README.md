# Solana Proof Store

A Solana on-chain program for **immutable** storage and verification of page metadata and content hashes to ensure data integrity validation.

## Features

### Enhanced Data Storage
- **Content Hash**: SHA-256 hash of page content for integrity verification
- **Metadata Hash**: SHA-256 hash of structured metadata for tamper detection  
- **Essential Metadata**: Store URL, title, content type, and size
- **Versioning**: Schema version support for future upgrades
- **Immutability**: Once stored, proofs cannot be modified (ensuring integrity)

### Program Instructions

#### 1. StoreProof
Store comprehensive page metadata and content hashes on-chain **permanently**.
```rust
StoreProof {
    url: String,
    content_hash: String,        // SHA-256 of content
    metadata_hash: String,       // SHA-256 of metadata JSON
    title: String,               // Page title
    content_type: String,        // MIME type
    content_length: u64,         // Content size in bytes
}
```

#### 2. GetProof
Retrieve and verify stored proof data.

#### 3. VerifyHash
Verify that provided content hash matches stored hash.

**Note**: No update operations are supported to maintain proof integrity.

## Deployment

### Building the Program
```bash
# Build using Docker
docker build -t proof-store .

# Or build locally with Solana CLI
cargo build-sbf --manifest-path Cargo.toml --sbf-out-dir target/deploy
```

### Deploy to Different Networks
```bash
# Deploy to devnet
solana program deploy target/deploy/proof_store.so --url devnet

# Deploy to testnet  
solana program deploy target/deploy/proof_store.so --url testnet

# Deploy to mainnet-beta (production)
solana program deploy target/deploy/proof_store.so --url mainnet-beta
```

## Usage Examples

### 1. Deploy to Solana Network
```bash
# Build the program
cargo build-sbf --manifest-path Cargo.toml --sbf-out-dir target/deploy

# Deploy to devnet
solana program deploy target/deploy/proof_store.so --url devnet

# Deploy to mainnet (when ready)
solana program deploy target/deploy/proof_store.so --url mainnet-beta
```

### 2. Store Page Proof (via transaction)
```javascript
// Example using @solana/web3.js
const instruction = {
  programId: PROOF_STORE_PROGRAM_ID,
  keys: [
    { pubkey: payer.publicKey, isSigner: true, isWritable: false },
    { pubkey: proofAccount.publicKey, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
  ],
  data: Buffer.from(borsh.serialize(ProofInstructionSchema, {
    storeProof: {
      url: "https://example.com/page",
      content_hash: "abc123...",
      metadata_hash: "def456...",
      title: "Page Title",
      content_type: "text/html",
      content_length: 1024
    }
  }))
};
```

### 3. Verify Stored Proof
```javascript
// Retrieve proof account data and verify
const accountInfo = await connection.getAccountInfo(proofAccount.publicKey);
const proofData = borsh.deserialize(ProofDataSchema, accountInfo.data);

// Generate explorer links for users
const txLink = `https://explorer.solana.com/tx/${signature}?cluster=devnet`;
const accountLink = `https://explorer.solana.com/account/${proofAccount.publicKey}?cluster=devnet`;
```

## Benefits

1. **Immutable Proof**: Leverage Solana's blockchain for tamper-proof storage
2. **Public Verification**: Anyone can verify proofs using explorer links
3. **Rich Metadata**: Store comprehensive information beyond just hashes
4. **Cost Effective**: Efficient storage using Borsh serialization
5. **Future Proof**: Versioned schema supports upgrades
6. **User Friendly**: Explorer integration makes verification accessible
7. **Trustless**: No central authority can modify stored proofs

## Security Considerations

- Use SHA-256 for cryptographically secure hashing
- Store both content and metadata hashes for complete verification
- **Immutability**: Once stored, proofs cannot be changed (by design)
- Consider rent-exempt account funding for permanent storage
- Create new proofs for legitimate content changes rather than updates

## License

MIT License 