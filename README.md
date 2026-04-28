# 🕵️ zk-SNARKs Protocol

A demonstration of Zero-Knowledge Proof (ZKP) concepts implemented in Rust. This repository focuses on the mathematical foundations of privacy-preserving protocols, specifically Schnorr-style identification and the concepts behind zk-SNARKs.

## 🧠 Concepts
- **Prover**: The entity that wants to prove knowledge of a secret without revealing it.
- **Verifier**: The entity that verifies the proof.
- **Completeness**: If the statement is true, an honest prover will convince an honest verifier.
- **Soundness**: If the statement is false, a cheating prover cannot convince an honest verifier.
- **Zero-Knowledge**: The verifier learns nothing other than the truth of the statement.

## 🚀 Getting Started

Ensure you have Rust and Cargo installed.

```bash
# Run the demonstration
cargo run
```

## 🛠 Features
- **Discrete Logarithm Proof**: Implementation of the Schnorr protocol concept.
- **Modular Arithmetic**: Hand-written modular exponentiation for cryptographic security.
- **Educational Flow**: Detailed console output explaining each step of the cryptographic handshake.