import { Injectable } from '@nestjs/common';

@Injectable()
export class VerifierService {
  getHello(): string {
    return 'Hello Verifier!';
  }
}
