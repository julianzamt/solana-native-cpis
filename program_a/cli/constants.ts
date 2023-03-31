import * as web3 from "@solana/web3.js";

export const SECRET = [48,94,163,75,125,167,37,186,99,172,218,55,115,47,207,161,225,244,13,52,186,136,56,161,12,201,131,153,35,139,105,176,204,25,224,73,185,238,107,68,240,170,192,158,58,99,193,135,163,76,243,235,147,10,7,19,45,24,159,10,148,60,249,57]; // your local private test-key. Try: $cat ~/.config/solana/id.json
export const PROGRAM_ID = "8bTxyQrL6aMmEsXxBKirmWfFVnLr87SKTCQrgb4krcLx"; // Whatever returned from solana deploy
export const PROGRAM_B_ID = "HyYjeFzX6evd8waHu9YmEWkJzxYX8ryAp2AnKF5jeeVJ";
export const KEY: Uint8Array = Uint8Array.from(SECRET);
export const programId = new web3.PublicKey(PROGRAM_ID);
export const programBId = new web3.PublicKey(PROGRAM_B_ID);
export const SIGNER: web3.Keypair = web3.Keypair.fromSecretKey(KEY);
