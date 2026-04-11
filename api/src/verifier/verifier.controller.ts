import { Controller, Get, Render, Query } from '@nestjs/common';
import * as qrcode from 'qrcode';
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

  @Get('qr')
  @Render('qr')
  async generateQrCode() {
    const hardcodedData = 'Hello, QR Code!'; // Hardcoded data
    const qrCodeDataUrl = await qrcode.toDataURL(hardcodedData);
    return { qrCodeDataUrl };
  }
}
