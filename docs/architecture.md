# ZeroAge - Technical Architecture
This document provides a comprehensive technical overview of the ZeroAge anonymous age-verification system. ZeroAge is a Proof-of-Concept EUDI Wallet that allows citizens to prove they are over 18 years old — without revealing any personal data, using Zero-Knowledge Proofs (ZKP) built on the Groth16 proving system over the BN254 elliptic curve.

## 1. High-Level System Overview
The system is composed of four principal components that interact through well-defined interfaces. The Wallet (mobile client) is always the active party that initiates proofs - it scans a Verifier's QR code, receives a cryptographic challenge, computes a ZKP locally, and returns the proof.

![High-level System Overview](diagrams/HighLevel_System_Overview.png)

| Component | Technology | Role |
|-----------|-----------|------|
| **Issuer Service** | Rust, Actix-web 4, SQLite, arkworks (BN254, Baby JubJub) | Issues digitally signed credentials to registered users |
| **Wallet App** | Tauri 2, SvelteKit 5, Rust, arkworks (Groth16/BN254), Android Keystore | Stores credentials, scans QR codes, generates ZK proofs on-device |
| **Verifier API** | Rust, Actix-web 4, arkworks (Groth16/BN254) | Generates challenges, validates submitted ZK proofs server-side |
| **Verifier UI** | HTMX 2.0, Askama templates, QR code (SVG) | In-store terminal that displays challenge QR and shows verification result |
| **ZK Module** | arkworks (R1CS, Groth16), Circom 2.0 (legacy circuit) | Defines the arithmetic circuit, proving keys, and verification keys |

## 2. Core Protocol — Challenge-Response with Zero-Knowledge Proofs
The fundamental design principle is that the client always scans the Verifier's QR code. The QR code encodes a cryptographic challenge that the Wallet must process through its local ZK circuit and return as a valid proof. This ensures freshness, prevents replay, and keeps the user in control.

### 2.1 Protocol Sequence

![Protocol Sequence](diagrams/Protocol_Sequence.png)

### 2.2 Challenge Structure
The QR code displayed by the Verifier terminal encodes a JSON payload:

```json
{
  "challengeId": "a3f8c2e1-...-uuid-v4",
  "nonce": "7a9b3c...128-bit-random-hex",
  "timestamp": 1712066400,
  "expiry": 1712066700,
  "callbackUrl": "http://10.0.2.2:3000/api/verifier/verify",
  "verifierName": "TestVerifier"
}
```

| Field | Type | Purpose |
|-------|------|---------|
| `challengeId` | UUID v4 | Unique session identifier for this verification attempt |
| `nonce` | 32-char hex (128-bit) | Cryptographic randomness bound into the ZKP to prevent replay |
| `timestamp` | Unix epoch (seconds) | Challenge creation time |
| `expiry` | Unix epoch (seconds) | Challenge expiration = timestamp + 5 minutes |
| `callbackUrl` | HTTPS URL | Endpoint where the Wallet POSTs the proof |
| `verifierName` | String | Human-readable name shown to the user for informed consent |

### 2.3 Nonce Lifecycle
The `Nonce` is a 16-byte value generated using `rand::rngs::StdRng`, serialized as a 32-character hex string. It serves 3 critical purposes:
- **Replay prevention** - each challenge has a unique nonce bound into the ZKP
- **Challenge-proof binding** - the nonce links the proof to a specific verification session
- **One-time use** - after verification (success, failure, or expiry), the challenge is removed from the in-memory `HashMap<Nonce, Challenge>`

## 3. Zero-Knowledge Circuit Architecture
The project contains two ZK circuit implementations: a **legacy Circom circuit** (`zk/circuits/ageVerifier.circom`) and the **active arkworks R1CS circuit** (`wallet/src-tauri/src/zkp_gen.rs`). The wallet uses the arkworks circuit for on-device proof generation, while the Circom circuit served as the original prototype.

### 3.1 Active Circuit: AgeVerifier (arkworks R1CS)
The production circuit is defined natively in Rust using the arkworks R1CS constraint framework. It proves that the holder is at least 18 years old **and** that the credential was genuinely signed by the trusted Issuer - without revealing any private data.

![Active Circuit - Age Verifier](diagrams/ActiveCircuit-AgeVerifier.png)

**What the circuit proves** (without revealing any private data):

| # | Constraint | Purpose |
|---|-----------|---------|
| 1 | `birthdate ≤ generation_date − 567,648,000` | User is at least 18 years old |
| 2 | `issue_date < generation_date` | Credential was issued before proof generation |
| 3 | `generation_date < expiry_date` | Credential has not expired |
| 4 | `s·G + e·PK == R` where `e = Poseidon(R.x, R.y, birthdate)` | Schnorr signature from the trusted Issuer is valid |

### 3.2 Legacy Circuit: StudentStatusVerifier (Circom)
The `zk/circuits/ageVerifier.circom` file contains the original prototype circuit built with Circom 2.0. It verifies both age (≥18) and active student status using an AND gate. The compiled artifacts (`circuit.wasm`, `circuit_final.zkey`, `verification_key.json`) remain in the repository but are not used by the current wallet or API.

![Legacy Circuit - StudentStatusVerifier](diagrams/LegacyCircuit-StudentStatusVerifier.png)

### 3.3 Trusted Setup Artifacts

| Artifact | Location | Purpose |
|----------|----------|---------|
| `pk.bin` | `wallet/src-tauri/assets/` | Groth16 proving key - embedded in wallet binary at compile time |
| `vk.bin` | `zk/artifacts/` | Groth16 verifying key - embedded in API binary at compile time |
| `issuer_pk.bin` | `wallet/src-tauri/assets/` | Issuer's Baby JubJub public key - baked into the wallet circuit |
| `issuer_sk.bin` | `issuer/assets/` (runtime) | Issuer's Baby JubJub private key - loaded by the Issuer at startup |
| `circuit.wasm` | `zk/artifacts/` | Legacy Circom WASM witness generator (not actively used) |
| `circuit_final.zkey` | `zk/artifacts/` | Legacy Circom proving key (not actively used) |
| `verification_key.json` | `zk/artifacts/` | Legacy Circom verification key (JSON, protocol: groth16, curve: bn128) |

## 4. Component Architecture

### 4.1 Wallet App (Tauri 2 / SvelteKit 5)
The Wallet is the user-facing mobile application. It is the sole entity that holds private credentials and generates ZK proofs. The architecture follows a layered design.

![Wallet_App](diagrams/Wallet_App.png)

**Tauri Command Registry** (7 commands):

| Command | Module | Description |
|---------|--------|-------------|
| `is_auth` | auth.rs | Check if user is authenticated (token in memory or keystore) |
| `login` | auth.rs | POST to Issuer `/login`, store token in memory + keystore |
| `register` | auth.rs | POST to Issuer `/register` with activation code |
| `logout` | auth.rs | Clear token + document from memory and keystore |
| `request_document` | zkp.rs | GET signed credential from Issuer `/document`, store locally |
| `load_document` | zkp.rs | Load credential from Android Keystore |
| `generate_proof` | zkp.rs | Generate Groth16 proof and POST to Verifier callback URL |

### 4.2 Verifier Service (Actix-web)
The Verifier serves both a REST API and an HTMX-driven web UI from a single binary.

![Verifier Service](diagrams/Verifier_Service.png)

**API Endpoints:**

| Method | Path | Request | Response |
|--------|------|---------|----------|
| `GET` | `/api/verifier/challenge` | - | `Challenge` JSON |
| `POST` | `/api/verifier/verify` | `{proof: string, public_inputs: {generation_date: u64, nonce: u128}}` | `{success: bool, message: string}` |
| `GET` | `/api/verifier/status?nonce=<hex>` | Query param | Plain text: `"success"` / `"failure"` / `"expired"` |

**UI Endpoints (HTMX partials):**

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/ui` | Full page, loads base.html, triggers `/ui/verification` |
| `GET` | `/ui/verification` | Landing partial with title + auto-triggers `/ui/challenge` |
| `GET` | `/ui/challenge` | Renders QR code (SVG) + challenge details + starts long-poll |
| `GET` | `/ui/status?nonce=<hex>` | Shows verification result (success / failure / expired) |

## 5. Credential Lifecycle
The credential lifecycle follows a four-phase process: **Provisioning -> Issuance -> Storage -> Verification**.

![Credential Lifecycle](diagrams/Credential_Lifecycle.png)

**UserDocument structure** (the issued credential):

```json
{
  "identifier": "uuid-v4",
  "firstName": "Jan",
  "lastName": "Kowalski",
  "dateOfBirth": 946684800,
  "issueDate": 1718000000,
  "expiryDate": 1720592000,
  "sigR": "base64-encoded BJJ affine point (Schnorr R)",
  "sigS": "base64-encoded Fr scalar (Schnorr s)"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `identifier` | UUID v4 | Unique document identifier |
| `firstName` / `lastName` | String | Citizen's name (from activation code) |
| `dateOfBirth` | u64 (Unix epoch) | Timestamp of birth - the signed attribute |
| `issueDate` | u64 (Unix epoch) | Current time at issuance |
| `expiryDate` | u64 (Unix epoch) | `issueDate + 30 days` |
| `sigR` | Base64 string | Schnorr signature R point (compressed BJJ affine) |
| `sigS` | Base64 string | Schnorr signature s scalar (compressed Fr) |

## 6. Security Architecture

### 6.1 Threat Mitigations

| Threat | Mitigation | Mechanism |
|--------|-----------|-----------|
| **Replay Attack** | One-time nonce per challenge | 128-bit random nonce bound into ZKP; challenge removed after use |
| **Credential Forgery** | Schnorr signature in circuit | Proof is valid only if Issuer's signature verifies inside the ZKP |
| **Data Leakage** | Zero-Knowledge | Verifier learns only `generation_date` + `nonce` + validity (1/0) |
| **Expired Credentials** | Expiry check in circuit | `generation_date < expiry_date` enforced as R1CS constraint |
| **Stolen Device** | Biometric + Keystore | Token access requires biometric auth; documents stored in hardware-backed keystore |
| **Challenge Exhaustion** | TTL + cleanup | 5-minute TTL; background task cleans expired challenges every 60 seconds |
| **Man-in-the-Middle** | Challenge binding | Proof is bound to specific `nonce` and `callbackUrl` from the QR code |

### 6.2 Privacy Guarantees

| Property | How It's Achieved |
|----------|-------------------|
| **Data Minimization** | Only `generation_date` and `nonce` are public inputs; no PII leaves the device |
| **Unlinkability** | Each verification uses a fresh nonce; no persistent identifiers transmitted |
| **Local-only Storage** | Credentials and tokens stored exclusively in Android Keystore on-device |
| **User Consent** | Wallet shows verifier name and URL; user must explicitly confirm before generating proof |
| **No Tracking** | Verifier stores challenges in-memory only (HashMap); removed after verification |












