## Keyless accounts
This repository contains a prototype implementation of keyless accounts defined in an Aptos improvement proposal found [here](https://github.com/aptos-foundation/AIPs/blob/main/aips/aip-61.md)


## Parts
- Prover service (delegate construction ZK proof)
- on-chain Verifier
- Pepper service

## Build

Make sure when building anything depending on Aptos SDK to run this when building

```rust
RUSTFLAGS="--cfg tokio_unstable"  cargo build --release
```
