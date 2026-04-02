export async function generateProof(userData: string): Promise<string | null> {
  if (userData) {
    return "0000000000"; // Here logic for generating proof
  }
  return;
}

export async function sendProof(
  userData: string,
  endpoint: string,
): Promise<boolean> {
  console.log("Proof sent");
  return false; // Place to handle logic when sending proof to endpoint.
}
