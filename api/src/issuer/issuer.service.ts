import { Injectable } from '@nestjs/common';

@Injectable()
export class IssuerService {
  getHello(): string {
    return 'Hello Issuer!';
  }
}
