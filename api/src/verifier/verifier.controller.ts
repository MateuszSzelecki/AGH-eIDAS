import { Controller, Get, Post, Render, Body, Res } from '@nestjs/common';
import type { Response } from 'express';
import * as qrcode from 'qrcode';
import { VerifierService } from './verifier.service';
import { ProofDto } from './dto/proof.dto';

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
    return { message: 'Weryfikator eIDAS' };
  }

  @Get('qr')
  @Render('qr')
  async generateQrCode() {
    // Generate challenge
    const challenge = this.verifierService.generateChallenge();

    console.log('Generated challenge:', challenge);

    // Create QR code with challenge
    const qrCodeContent = JSON.stringify(challenge);
    const qrCodeDataUrl = await qrcode.toDataURL(qrCodeContent);

    return {
      qrCodeDataUrl,
      challenge: JSON.stringify(challenge, null, 2),
    };
  }

  @Post('verify')
  async verifyProof(
    @Body() body: { proof: ProofDto; nonce: string },
    @Res() res: Response,
  ) {
    console.log('Received proof verification request');
    console.log('Nonce:', body.nonce);
    console.log('Proof:', JSON.stringify(body.proof, null, 2));

    try {
      const isValid = await this.verifierService.verifyProof(
        body.proof,
        body.nonce,
      );

      if (isValid) {
        console.log('✅ Proof verified successfully!');
        return res.status(200).json({
          success: true,
          message: 'Verification successful',
        });
      } else {
        console.log('❌ Proof verification failed');
        return res.status(400).json({
          success: false,
          message: 'Verification failed',
        });
      }
    } catch (error) {
      console.error('Error during verification:', error);
      return res.status(500).json({
        success: false,
        message: 'Internal server error',
      });
    }
  }
}
