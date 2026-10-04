import { AnchorProvider, Program, Wallet, type Idl } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { createEnv } from "@t3-oss/env-core";
import { z } from "zod";

const commitment = "confirmed";
const env = createEnv({
  server: {
    SOLANA_RPC_URL: z.string().url(),
    KEYPAIR_PATH: z.string().min(1),
  },
  runtimeEnv: Bun.env,
  emptyStringAsUndefined: true,
});

export type TestContext = {
  connection: Connection;
  payer: Keypair;
  program: Program;
  provider: AnchorProvider;
};

export async function createTestContext(): Promise<TestContext> {
  const payer = await readKeypair(env.KEYPAIR_PATH);
  const connection = new Connection(env.SOLANA_RPC_URL, commitment);
  const provider = new AnchorProvider(connection, new Wallet(payer), { commitment });
  const idlPath = new URL("../target/idl/vrf_demo.json", import.meta.url);
  const idl = (await Bun.file(idlPath).json()) as Idl;

  return {
    connection,
    payer,
    program: new Program(idl, provider),
    provider,
  };
}

async function readKeypair(path: string): Promise<Keypair> {
  const secretKey = await Bun.file(path).json();
  if (!Array.isArray(secretKey)) {
    throw new Error(`Expected a Solana keypair array in ${path}`);
  }
  return Keypair.fromSecretKey(Uint8Array.from(secretKey));
}
