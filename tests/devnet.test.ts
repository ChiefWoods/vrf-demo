import { test } from "bun:test";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { SecretKey } from "@blueshift-gg/solana-ecvrf";
import { Orao, networkStateAccountAddress, randomnessAccountAddress } from "@orao-network/solana-vrf";
import { AnchorUtils, ON_DEMAND_DEVNET_QUEUE, Randomness } from "@switchboard-xyz/on-demand";

import { createTestContext, runAndLogRandomness } from "./common.ts";

// MagicBlock's base-layer default queue. The current TypeScript SDK does not
// expose VRF queue constants; this must match `vrf::consts::DEFAULT_QUEUE`.
const MAGICBLOCK_BASE_LAYER_VRF_QUEUE = new PublicKey(
  "Cuj97ggrhhidhbu39TijNVqE74xvKJ69gDervRUXAxGh",
);

test("MagicBlock", async () => {
  const { connection, payer, program } = await createTestContext();
  const seed = Array.from(Keypair.generate().publicKey.toBytes());

  await runAndLogRandomness(connection, program.programId, "MagicBlock VRF randomness", () =>
    program.methods.requestMagicblock!(seed)
      .accounts({
        payer: payer.publicKey,
        oracleQueue: MAGICBLOCK_BASE_LAYER_VRF_QUEUE,
      })
      .rpc(),
  );
});

test("Switchboard On-Demand", async () => {
  const context = await createTestContext();
  const sbProgram = await AnchorUtils.loadProgramFromConnection(
    context.connection,
    context.provider.wallet as any,
  );
  const queue = ON_DEMAND_DEVNET_QUEUE;
  const randomnessKeypair = Keypair.generate();
  const [randomness, createRandomness] = await Randomness.create(
    sbProgram,
    randomnessKeypair,
    queue,
    context.payer.publicKey,
  );

  await context.provider.sendAndConfirm(new Transaction().add(createRandomness), [
    randomnessKeypair,
  ]);

  const consume = await context.program.methods.consumeSwitchboardOnDemand!()
    .accounts({ randomness: randomness.pubkey })
    .instruction();

  await runAndLogRandomness(
    context.connection,
    context.program.programId,
    "Switchboard On-Demand randomness",
    () => randomness.commitAndReveal([consume], [context.payer], queue),
  );
});

test("Orao VRF", async () => {
  const { connection, payer, program, provider } = await createTestContext();
  const seed = Keypair.generate().publicKey.toBytes();
  const vrf = new Orao(provider);
  const networkState = networkStateAccountAddress();
  const request = randomnessAccountAddress(seed);
  const { config } = await vrf.getNetworkState();

  await runAndLogRandomness(connection, program.programId, "Orao VRF randomness", async () => {
    await program.methods.requestOrao!(Array.from(seed))
      .accounts({
        payer: payer.publicKey,
        networkState,
        treasury: config.treasury,
        request,
        vrf: vrf.programId,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    await vrf.waitFulfilled(seed);
    return program.methods.consumeOrao!().accounts({ request }).rpc();
  });
}, 120_000);

test("Blueshift ECVRF", async () => {
  const { connection, program } = await createTestContext();
  const keypair = Keypair.generate();
  const secretKey = SecretKey.fromKeypair(keypair.secretKey);
  const alpha = Uint8Array.from(Buffer.from("vrf-demo:blueshift-ecvrf"));
  const proof = secretKey.prove(alpha);

  // Prove the client and on-chain verifier agree before submitting the proof.
  proof.verify(secretKey.publicKey, alpha);

  await runAndLogRandomness(connection, program.programId, "Blueshift ECVRF randomness", () =>
    program.methods.verifyBlueshiftEcvrf!(
      Array.from(secretKey.publicKey.bytes),
      Array.from(proof.bytes),
      Buffer.from(alpha),
    )
      .accounts({ systemProgram: SystemProgram.programId })
      .rpc(),
  );
});
