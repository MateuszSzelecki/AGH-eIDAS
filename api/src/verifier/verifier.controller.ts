import { Controller, Get, Render } from '@nestjs/common';
import * as qrcode from 'qrcode';
import { VerifierService } from './verifier.service';
import * as crypto from 'crypto';

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
    const birthDate = new Date('2000-01-01T00:00:00Z'); // Example: January 1, 2000 UTC
    const dateOfBirth = Math.floor(birthDate.getTime() / 1000).toString();

    const isActiveStudent = '1';

    const now = new Date();
    const dateToday = Math.floor(now.getTime() / 1000).toString();

    const nonceHex = crypto.randomBytes(32).toString('hex');
    const nonce = BigInt('0x' + nonceHex).toString();

    const { isValid, nonceEcho } =
      await this.verifierService.verifyAgeAndStudentStatus(
        dateOfBirth,
        isActiveStudent,
        dateToday,
        nonce,
      );

    const qrCodeContent = JSON.stringify({ isValid, nonceEcho });
    console.log('QR Code content:', qrCodeContent);

    const qrCodeDataUrl = await qrcode.toDataURL(qrCodeContent);
    return { qrCodeDataUrl };
  }
}
