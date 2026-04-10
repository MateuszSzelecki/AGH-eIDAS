# ZeroAge - Technical Architecture
This document provides a comprehensive technical overview of the ZeroAge anonymous age-verification system. ZeroAge is a Proof-of-Concept EUDI Wallet that allows citizens to prove they are over 18 years old, and optionally that they hold active student status — without revealing any personal data, using Zero-Knowledge Proofs (ZKP) built on the Groth16 proving system.

## 1. High-Level System Overview
The system is composed of four principal components that interact through well-defined interfaces. The Wallet (mobile client) is always the active party that initiates proofs, it scans a Verifier's QR code, receives a cryptographic challenge, computes a ZKP locally, and returns the proof.

<img width="763" height="684" alt="Schemat1" src="https://github.com/user-attachments/assets/00fd7e7f-8bfa-4742-8491-0789aaac79fd" />

| Component | Technology | Role |
|-----------|-----------|------|
| **Issuer Service** | NestJS, TypeScript | Issues digitally signed credentials to registered users |
| **Wallet App** | React Native, Expo, expo-secure-store | Stores credentials, scans QR codes, generates ZK proofs on-device |
| **Verifier Service** | NestJS, TypeScript | Generates challenges, validates submitted ZK proofs |
| **QR Verifier** | React, browser camera API, snarkjs | In-store terminal that displays challenge QR and shows verification result |
| **ZK Module** | Circom 2.0, snarkjs, circomlib | Defines the arithmetic circuit, proving keys, and verification keys |

## 2. Core Protocol - Challenge-Response with Zero-Knowledge Proofs
The fundamental design principle is that the client always scans the Verifier's QR code. The QR code encodes a cryptographic challenge that the Wallet must process through its local ZK circuit and return as a valid proof. This ensures freshness, prevents replay, and keeps the user in control.

### 2.1 Protocol Sequence
<img width="1389" height="1165" alt="Schemat2" src="https://github.com/user-attachments/assets/6ac06b11-5665-4637-9658-773cd6e09b79" />

### 2.2 Challenge Structure
The QR code displayed by the Verifier terminal encodes a JSON payload:
<pre>
{
  "challengeId": "a3f8c2e1-uuid",
  "nonce": "7a9b3c...128-bit-random-hex",
  "timestamp": 1712066400,
  "callbackUrl": "https://api.zeroage.io/verifier/verify",
  "verifierName": "SuperMarket #42"
}
</pre>

| Field | Purpose |
|-------|---------|
| challengeId | Unique session identifier for this verification attempt |
| nonce | Cryptographic randomness bound into the ZKP to prevent replay |
| timestamp | Unix time of challenge creation, used for TTL enforcement |
| callbackUrl | HTTPS endpoint where the Wallet submits the proof |
| verifierName | Human-readable name shown to the user for informed consent - for  group of verifiers |

## 3. Zero-Knowledge Circuit Architecture
The ZK module uses Circom 2.0 to define an arithmetic circuit compiled into the Groth16 proving system. The circuit proves two predicates simultaneously without revealing the underlying data.

### 3.1a Age Verification Circuit (>= 18)
The simplest circuit. It takes the user's date of birth as a private input and today's date as a public input, then proves that the user is at least 18 years old without disclosing the actual birth date.

<img width="1051" height="249" alt="Schemat3" src="https://github.com/user-attachments/assets/1428c642-450c-40c5-be30-c93965181a87" />

What the Verifier learns: Only whether isOver18 == 1. The exact date of birth remains hidden inside the proof.

### 3.1b Age + Student Status Circuit
The extended circuit adds a second private input (isActiveStudent) and combines both predicates with an AND gate. The proof is valid only if the user is both over 18 and an active student.

<img width="1386" height="417" alt="Schemat4" src="https://github.com/user-attachments/assets/f9348fe3-145d-4982-bfd2-de4ba7486635" />

What the Verifier learns: Only whether isValid == 1 (age ≥ 18 AND active student). Neither the birth date nor the student ID are revealed - the boolean constraint on isActiveStudent enforces that the input can only be 0 or 1, preventing any information leakage through the field element.

### 3.3 Trusted Setup Artifacts
The ZK module produces three artifacts during the trusted setup ceremony:

| Artifact              | Purpose                                                                            |
| --------------------- | ---------------------------------------------------------------------------------- |
| circuit.wasm          | WebAssembly prover, computes the witness on-device                                 |
| circuit_final.zkey    | Proving key, used by snarkjs to generate the Groth16 proof                         |
| verification_key.json | Verification key, used server-side (and optionally client-side) to validate proofs |

## 4. Component Architecture

### 4.1 Wallet App (React Native / Expo)
The Wallet is the user-facing mobile application. It is the sole entity that holds private credentials and generates ZK proofs. The architecture follows a layered design.

## 5. Credential Lifecycle

## 6. Security Architecture

## 7. Technology Stack Summary

## 8. Deployment Topology








