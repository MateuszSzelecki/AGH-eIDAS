import { Injectable } from '@nestjs/common';
import * as snarkjs from 'snarkjs';
import * as path from 'path';
import * as crypto from 'crypto';
import * as fs from 'fs';
import { buildPoseidon } from 'circomlibjs';
import { ChallengeDto } from './dto/challenge.dto';
import { ProofDto } from './dto/proof.dto';

@Injectable()
export class VerifierService {
  private verificationKeyPath: string;
  private challenges: Map<string, ChallengeDto> = new Map();
  private verifiedNonces: Set<string> = new Set();

  constructor() {
    this.verificationKeyPath = path.join(
      __dirname,
      '../../../zk/artifacts/verification_key.json',
    );
  }

  getHello(): string {
    return 'Hello Verifier!';
  }

  generateChallenge(): ChallengeDto {
    // Generate random 128-bit nonce (16 bytes = 128 bits)
    const nonceBytes = crypto.randomBytes(16);
    const nonce = nonceBytes.toString('hex');

    const challenge: ChallengeDto = {
      challengeId: crypto.randomUUID(),
      nonce,
      timestamp: Math.floor(Date.now() / 1000),
      callbackUrl: 'http://10.0.2.2:3000/verifier/verify',
      verifierName: 'TestVerifier',
    };

    // Store challenge for later verification
    this.challenges.set(nonce, challenge);

    return challenge;
  }

  getVerificationStatus(nonce: string): 'pending' | 'verified' | 'unknown' {
    if (this.verifiedNonces.has(nonce)) return 'verified';
    if (this.challenges.has(nonce)) return 'pending';
    return 'unknown';
  }

  async verifyProof(proof: ProofDto, nonce: string): Promise<boolean> {
    try {
      // Check if challenge exists
      const challenge = this.challenges.get(nonce);
      if (!challenge) {
        console.error('Challenge not found for nonce:', nonce);
        return false;
      }

      // Load verification key
      const vKey = JSON.parse(
        fs.readFileSync(this.verificationKeyPath, 'utf-8'),
      );

      // Calculate expected Poseidon hash of nonce
      const poseidon = await buildPoseidon();
      const nonceHash = poseidon.F.toString(
        poseidon([BigInt('0x' + nonce)]),
      );

      console.log('Expected nonce hash:', nonceHash);
      console.log('Received publicSignals:', proof.publicSignals);

      // Verify the proof
      const isValidProof = await snarkjs.groth16.verify(
        vKey,
        proof.publicSignals,
        proof,
      );

      // Check conditions:
      // 1. Proof is cryptographically valid
      // 2. publicSignals[0] === "1" (user meets age requirement) - może być usunięte w przyszłości
      // 3. publicSignals[1] === nonceHash (nonce matches - anti-replay)
      const isValid =
        isValidProof &&
        proof.publicSignals[0] === '1' &&
        proof.publicSignals[1] === nonceHash;

      if (isValid) {
        this.challenges.delete(nonce);
        this.verifiedNonces.add(nonce);
        // Clean up verified nonce after 5 minutes
        setTimeout(() => this.verifiedNonces.delete(nonce), 5 * 60 * 1000);
      }

      return isValid;
    } catch (error) {
      console.error('Error during proof verification:', error);
      return false;
    }
  }
}
