# eIDAS 2.0 and EUDI ARF Technical Requirements

This document gathers the most important technical and legal guidelines necessary for integrating digital services (Issuers and Verifiers) with **European Digital Identity (EUDI) Wallets** under the **eIDAS 2.0** regulation. Meeting these requirements is essential for moving from a closed, local solution to becoming part of a pan-European identity ecosystem.

---

## 1. Standardization Foundations (Sources of Requirements)

The entire technical architecture rests on two pillars:
*   [Regulation (EU) 2024/1183 (eIDAS 2.0)](https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202401183): The legal act specifying, among other things, the Level of Assurance requirements (LoA High) and free access to wallets for EU citizens.
*   [EUDI Architecture and Reference Framework (ARF)](https://github.com/eu-digital-identity-wallet): Technical documentation created by the European Commission that translates legal principles into specific programming protocols and algorithms. It strictly dictates how EUDI Wallets can receive and send data.

## 2. Official eIDAS 2.0 High-Level Requirements (HLR)

The EUDI ARF Annex 2 defines strict [High-Level Requirements (HLR)](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/annexes/annex-2/annex-2.02-high-level-requirements-by-topic.md) for wallets and relying parties (OIA - Online Identity Access and PID - Person Identification Data). Some of the most critical include:

### A. Presentation & Data Formats
*   **PID_02 & PID_03:** A PID Provider (e.g., a national government) SHALL issue the PID in both formats concurrently: ISO/IEC 18013-5 (mDoc) and SD-JWT VC. If a PID credential includes a portrait, it MUST be a pure JPEG image without any additional binary headers, adhering strictly to the ISO/IEC 19794-5 quality standards for a "Full Frontal Image".
*   **OIA_01 & OIA_04:** Wallets must support OpenID4VP for remote presentation flows and ISO/IEC 18013-5 (mDoc) for proximity (offline) presentations.
*   **Topic 18 & RPI_09 (Combined Presentation of Attributes):** Although the formal HLRs for Topic 18 are still being drafted, the framework requires Wallets and Relying Parties to support combining attributes from multiple distinct attestations (e.g., pulling your Date of Birth from a PID and your Student Status from a university diploma) into a single, unified presentation. Intermediaries and Relying Parties must cryptographically verify these combined presentations.
*   **OIA_08 & OIA_08a-e:** Integration with the W3C Digital Credentials API is strongly recommended for remote flows, provided it respects privacy (e.g., hiding exact attribute values from the browser/OS).

### B. Hardware & Cryptographic Security
*   **OIA_03:** All actors (Wallets, Issuers, Verifiers) must exclusively use algorithms approved in the ENISA ECCG Agreed Cryptographic Mechanisms v2.0.
*   **WIAM_01 (Wallet Instance Authentication & Management):** The mechanisms used to unlock the Wallet device locally (e.g., PIN, FaceID, fingerprint) must be cryptographically robust and actively evaluated to meet the strict eIDAS Level of Assurance High (LoA High) standards.
*   **OIA_02:** Wallets must prove cryptographic device binding between the wallet's secure hardware (WSCA/WSCD/keystore) and the attestation (Key Binding in SD-JWT VC / mdoc authentication).
*   **QES_01 & QES_02 (Qualified Electronic Signatures):** Wallets must support the capability for users to securely generate Qualified Electronic Signatures (QES) on documents (whether locally or via a Remote QTSP). A QES created via the EUDI wallet has the equivalent legal effect of a handwritten signature across the entire EU.

### C. Issuance & Lifecycle Management
*   **ISSU_01 (OpenID4VCI Issuance):** The architecture mandates using OpenID4VCI for issuing credentials. Common flows include the Authorization Code Flow (e.g., logging in via a national identity node to fetch a PID) and the Pre-Authorized Code Flow (e.g., scanning a QR code received from a university to retrieve an e-Student ID).
*   **WUA_03 & WUA_09 (Wallet Unit Attestation):** The Wallet must present a cryptographic proof (WUA) to the Issuer proving that the keys were genuinely generated inside the smartphone's Secure Enclave, preventing fake wallets from receiving valid credentials.
*   **VCR_11 (Credential Revocation):** Issuers must support a mechanism for instant revocation of active credentials (e.g., an expelled student). The ecosystem typically uses Token Status Lists (such as IETF Bitstring Status Lists), allowing Verifiers to check the validity status without revealing the user's identity.
*   **WURevocation_01 & WURevocation_11 (Wallet Unit Revocation):** Aside from revoking individual credentials, Issuers (and Users) must have a mechanism to instantly revoke the *entire Wallet Unit* (e.g., if the physical smartphone is lost or stolen), which immediately prevents any transactions originating from that specific device.

### D. Privacy, Tracking Protection & Transparency
*   **OIA_05 & OIA_06 & OIA_07:** The User must be shown exactly who is requesting data and what attributes are requested. The Wallet must implement Selective Disclosure and require explicit User approval before sending any data.
*   **PA_01 & PA_15 (Pseudonym Authentication):** Wallets must enable Users to generate and authenticate using cryptographic pseudonyms. This ensures Relying Parties cannot derive the User's true identity from the pseudonym, promoting absolute privacy and *Zero-Knowledge* principles.
*   **ISSU_37 & ISSU_43 (Batch Issuance & Anti-tracking):** To prevent correlation by colluding verifiers, Issuers should support Batch Issuance of attestations (e.g., issuing 50 copies of a credential at once). The Wallet then uses *Once-only attestations* (Method A), discarding each credential after a single presentation and renewing the batch automatically in the background.
*   **OIA_16:** Relying Parties must immediately discard any unique tracking elements/identifiers from the wallet after processing, enforcing anti-tracking/privacy.
*   **UD_01 (User Dashboard & Log Transparency):** The Wallet Unit must provide a local, user-friendly interface (dashboard) that persistently records and displays a log of all past data-sharing transactions, showing what attributes were presented, with whom, and exactly when.

### E. Trust & Relying Party Validation
*   **RPR_01 & RPR_02 (Entity Registration & Vetting):** The ecosystem is strictly regulated. All PID Providers, Attestation Providers (e.g., universities), and Relying Parties (Verifiers) must be formally registered in a recognized national registry (Registrar). An entity cannot participate in the ecosystem without obtaining a valid Registration Certificate.
*   **RPA_01 (Relying Party Authentication):** Verifiers cannot be anonymous. The Relying Party application must prove its identity to the Wallet upon requesting data, typically by signing the request with a recognized X.509 certificate from a national Trust Framework.
*   **OIA_12 - OIA_15:** Relying Parties must validate attestations using appropriate Trust Anchors (e.g., PID Provider LoTE or QEAA Trusted Lists) defined by the European Digital Identity Regulation.
*   **OIA_09:** For remote flows, presentation responses must be encrypted so that only the requesting Relying Party can access the attributes.
*   **SUA_01 (Strong User Authentication for Payments):** The EUDI Wallet is mandated to support Strong Customer Authentication (SCA) specifically designed for authorizing electronic payments and banking transactions under PSD2/PSD3 directives.
