import RustGenerateProof from "@/modules/rust-generate-proof";
import { UserId } from "@/services/storageService";

export interface ChallengePayload {
  challengeId: string;
  nonce: string;
  timestamp: number;
  callbackUrl: string;
  verifierName: string;
}

export function validateChallengePayload(json: string): ChallengePayload {
  let parsed: unknown;

  try {
    parsed = JSON.parse(json);
  } catch {
    throw new Error("Invalid JSON");
  }

  if (
    !(
      typeof parsed === "object" &&
      parsed !== null &&
      typeof parsed.challengeId === "string" &&
      typeof parsed.nonce === "string" &&
      typeof parsed.timestamp === "number" &&
      typeof parsed.callbackUrl === "string" &&
      typeof parsed.verifierName === "string"
    )
  ) {
    throw new Error("Invalid ChallengePayload structure");
  }

  if (!/^[a-f0-9]{32}$/i.test(parsed.nonce)) {
    console.log(parsed.nonce);
    throw new Error("Invalid nonce format");
  }

  if (!Number.isInteger(parsed.timestamp) || parsed.timestamp <= -1) {
    throw new Error("Invalid timestamp");
  }

  try {
    new URL(parsed.callbackUrl);
  } catch {
    throw new Error("Invalid callback URL");
  }

  let payload: ChallengePayload = {
    challengeId: parsed.challengeId,
    nonce: parsed.nonce,
    timestamp: parsed.timestamp,
    callbackUrl: parsed.callbackUrl,
    verifierName: parsed.verifierName,
  };

  return payload;
}

export async function generateProof(
  userData: UserId,
  challengePayload: ChallengePayload,
): Promise<string | null> {
  // rust library test
  console.log(await RustGenerateProof.rustAdd(1, 1));
  if (userData && challengePayload) {
    return await RustGenerateProof.generateProof(
      userData.dateBirth,
      true,
      Date.now(),
      challengePayload.nonce,
    );
  }

  return null;
}

export async function sendProof(
  userData: UserId,
  challengePayload: ChallengePayload,
): Promise<boolean> {
  console.log(userData);
  let nonce = challengePayload.nonce;
  let endpoint = challengePayload.callbackUrl;
  console.log(nonce, endpoint);
  let proof = await RustGenerateProof.generateProof(
    userData.dateBirth,
    true,
    Date.now(),
    nonce,
  );

  await fetch(endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: proof,
  })
    .then((response) => response.json())
    .then((data) => {
      console.log("Data sent successfully:", data);
      return true;
    })
    .catch((error) => {
      console.error("Error sending data:", error);
      return false;
    });

  return false;
}
