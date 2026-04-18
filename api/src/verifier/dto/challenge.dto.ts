export interface ChallengeDto {
  challengeId: string;
  nonce: string;
  timestamp: number;
  callbackUrl: string;
  verifierName: string;
}
