import { Module } from '@nestjs/common';
import { IssuerService } from './issuer.service';
import { IssuerController } from './issuer.controller';

@Module({
  controllers: [IssuerController],
  providers: [IssuerService],
})
export class IssuerModule {}
