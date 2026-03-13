## General Overview 

This repository contains the full implementation of the ZeroAge anonymous age verification system, including backend services, frontend applications, and Zero-Knowledge Proof circuits.

The project is organized as a monorepo to simplify development, integration, and deployment.

## Components Oveview

### Issuer API

The Issuer Service is responsible for issuing signed credentials based on student identity data.

Responsibilities:
- student registry
- credential issuance
- cryptographic signing
- revocation registry
- public key endpoint

Technology stack:
- Python
- FastAPI
- PostgreSQL / SQLite

### Verification API

The Verification Service handles verification of one-time codes used on websites.

Responsibilities:
- mapping verification codes to proofs
- proof validation
- TTL management
- anti-replay protection

Technology stack:
- Python
- FastAPI
- Redis

### Wallet (PWA)

The Wallet App is used by the user to store credentials and generate Zero-Knowledge Proofs.

Capabilities:
- secure credential storage
- biometric authentication (WebAuthn)
- ZKP proof generation (WASM)
- QR code generation
- one-time verification codes

Technology stack:
- React
- Vite
- WebAuthn
- IndexedDB

### QR Verifier

The Store Verifier App is used by physical merchants to verify age using a QR code.

Capabilities:
- camera-based QR scanning
- local proof verification
- simple visual result interface (OK / FAIL)

Technology stack:
- React
- browser camera APIs
- snarkjs verification

### ZK Module

The ZK module contains the Zero-Knowledge Proof circuits and artifacts.

Contents:
- circom circuit definitions
- compiled WASM prover
- proving keys
- verification keys
- trusted setup scripts

### Documentation

The docs/ directory contains detailed project documentation:
- system architecture
- cryptographic protocol
- threat model
- privacy and DPIA analysis