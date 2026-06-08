# Project Log & Meeting Minutes

This document tracks the progress, key decisions, and tasks discussed during the team's regular sync meetings.

---

### Meeting: Project Kickoff & Scope Definition
**Date:** March 6, 2026
**Topic:** Brainstorming, theme selection, and initial project scope.
**Key Decisions:**
* Selected the implementation of the eIDAS 2.0 framework as the core project theme.
* Decided to build a Proof of Concept (PoC) focused on privacy-preserving Age Verification (18+) using Zero-Knowledge concepts.

---

### Meeting: Architecture Sync & Feature Planning
**Date:** March 20, 2026
**Topic:** System architecture, user onboarding, and database constraints.
**Key Decisions:**
* Outlined the initial concept for credential issuance (mimicking the "mObywatel" flow).
* Confirmed that verification will be done via QR code scanning.

---

### Meeting: Threat Modeling & Security Mechanisms
**Date:** April 10, 2026
**Topic:** Threat intelligence, mitigating Replay Attacks, and scope reduction.
**Key Decisions:**
* Identified a vulnerability with static QR codes. Decided to implement a Challenge-Response mechanism with dynamic QR codes.
* Dropped the "Student Status" credential to strictly focus on Boolean Age Verification (`Age > 18`).

---

### Meeting: Supervisor Review (Dr inż. Jan Derkacz)
**Date:** April 23, 2026
**Topic:** Project presentation, architecture alignment, and feedback.
**Key Decisions:**
* Presented the current progress and the consolidated single-app PoC architecture.
* Clarified how the application conceptually maps to eIDAS 2.0 Selective Disclosure requirements without requiring complex external POS integration.

---

### Meeting: Technology Pivot 
**Date:** April 24, 2026
**Topic:** Refactoring cryptography and consolidating the application.
**Key Decisions:**
* Abandoned Circom/SnarkJS. Decided to use native Rust libraries (Schnorr signatures) for cryptographic proofs.
* Confirmed the merge of the User Wallet and Verifier Shop into a single frontend to simplify the demonstration.

---

### Meeting: Sprint Task Allocation
**Date:** May 14, 2026
**Topic:** Assigning remaining blockers before the final presentation.
**Key Decisions:**
* Verified that the API successfully accepts proofs from the app.
* Defined the final checklist of features required for the PoC completion.

**Action Items (Tasks to be completed):**
* Finalize application appearance and UI/UX flows.
* Implement server modifications and web frontend for the challenge display.
* Implement login and registration logic.
* Implement the Issuer logic (document signing).
* Finalize the main `Architecture.md` documentation.
* Conduct R&D on NFC proximity verification (as a future enhancement).