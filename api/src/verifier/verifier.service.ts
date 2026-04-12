import { Injectable } from '@nestjs/common';
import * as snarkjs from 'snarkjs';
import * as path from 'path';

@Injectable()
export class VerifierService {
  private wasmPath: string;
  private zkeyPath: string;

  constructor() {
    this.wasmPath = path.join(__dirname, '../../../zk/artifacts/circuit.wasm');
    this.zkeyPath = path.join(
      __dirname,
      '../../../zk/artifacts/circuit_final.zkey',
    );
  }

  getHello(): string {
    return 'Hello Verifier!';
  }

  async verifyAgeAndStudentStatus(
    DateOfBirth: string,
    isActiveStudent: string,
    DateToday: string,
    nonce: string,
  ): Promise<{ isValid: string; nonceEcho: string }> {
    const input = {
      DateOfBirth: DateOfBirth,
      isActiveStudent: isActiveStudent,
      DateToday: DateToday,
      nonce: nonce,
    };

    console.log('Input to WASM:', input);

    try {
      const { publicSignals } = await snarkjs.groth16.fullProve(
        input,
        this.wasmPath,
        this.zkeyPath,
      );

      const isValid = publicSignals[0];
      const nonceEcho = publicSignals[1];

      return { isValid, nonceEcho };
    } catch (error) {
      console.error('Error during WASM execution or proof generation:', error);
      throw new Error('Failed to verify age and student status.');
    }
  }
}
