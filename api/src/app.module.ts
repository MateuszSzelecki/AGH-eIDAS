import { Module } from '@nestjs/common';
import { IssuerModule } from './issuer/issuer.module';
import { VerifierModule } from './verifier/verifier.module';

@Module({
  imports: [IssuerModule, VerifierModule],
  controllers: [],
  providers: [],
})
export class AppModule {}
