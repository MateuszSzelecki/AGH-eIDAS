import { Controller, Get, Render } from '@nestjs/common';
import { VerifierService } from './verifier.service';

@Controller('verifier')
export class VerifierController {
  constructor(private readonly verifierService: VerifierService) {}

  @Get()
  getHello(): string {
    return this.verifierService.getHello();
  }

  @Get('index')
  @Render('index')
  root() {
    return { message: 'Hello world!' };
  }
}
