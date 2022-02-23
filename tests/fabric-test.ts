const fs = require("fs");
const assert = require("assert");
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

import { FabricTest } from "../target/types/fabric_test";

const idl = JSON.parse(fs.readFileSync("./target/idl/fabric_test.json", "utf8"));
const programId = new anchor.web3.PublicKey("2BG8YKnnFPq9KMEK9APRchysCHau86nC1iRig5j6TJVT");

describe("fabric-test", () => {
  console.log("Starting test...");
  // Configure the client to use the local cluster.
  const payer = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();

  console.log("Payer key: ", payer.publicKey.toString());
  console.log("User key: ", user.publicKey.toString());

  const connection = new anchor.web3.Connection("https://api.devnet.solana.com", "processed");
  const provider = new anchor.Provider(connection, new anchor.Wallet(payer), {
    skipPreflight: false,
    preflightCommitment: "processed",
    commitment: "processed",
  });
  anchor.setProvider(provider);
  const program = new Program<FabricTest>(idl, programId);

  const publicConnection = new anchor.web3.Connection("https://api.devnet.solana.com", { commitment: "processed" });

  const SYSTEM_PROGRAM = anchor.web3.SystemProgram.programId;
  const TOKEN_PROGRAM_ID = anchor.Spl.token().programId;
  const LAMPORTS_PER_SOL = anchor.web3.LAMPORTS_PER_SOL;

  const wrappedSol = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112");

  const wrappedSOLMintAcc = new Token(publicConnection, wrappedSol, TOKEN_PROGRAM_ID, payer);
  let redeemableMintAcc: Token;

  const initialAmount = 0.5;

  let userWrappedSol: anchor.web3.PublicKey, userRedeemableToken: anchor.web3.PublicKey;

  it("Initialize states!", async () => {
    await publicConnection.confirmTransaction(
      await publicConnection.requestAirdrop(payer.publicKey, 1.0 * LAMPORTS_PER_SOL),
      "finalized"
    );

    await publicConnection.confirmTransaction(
      await publicConnection.requestAirdrop(user.publicKey, 1.0 * LAMPORTS_PER_SOL),
      "finalized"
    );

    // userWrappedSol = await Token.getAssociatedTokenAddress(
    //   ASSOCIATED_TOKEN_PROGRAM_ID,
    //   TOKEN_PROGRAM_ID,
    //   wrappedSol,
    //   user.publicKey
    // );

    // const transaction = new anchor.web3.Transaction()
    //   .add(
    //     anchor.web3.SystemProgram.transfer({
    //       fromPubkey: user.publicKey,
    //       toPubkey: userWrappedSol,
    //       lamports: initialAmount * LAMPORTS_PER_SOL,
    //     })
    //   )
    //   .add(
    //     Token.createAssociatedTokenAccountInstruction(
    //       ASSOCIATED_TOKEN_PROGRAM_ID,
    //       TOKEN_PROGRAM_ID,
    //       wrappedSol,
    //       userWrappedSol,
    //       user.publicKey,
    //       payer.publicKey
    //     )
    //   );

    // anchor.web3.sendAndConfirmTransaction(provider.connection, transaction, [user, payer]);

    console.log("Wrapped 0.5 SOL to be used as SPL token");
  });

  // const poolName = "test_pool_" + Math.random().toString(16).substring(2, 8);
  // let pool: anchor.web3.PublicKey,
  //   poolBump: number,
  //   poolAuthority: anchor.web3.PublicKey,
  //   poolAuthorityBump: number,
  //   stakingVault: anchor.web3.PublicKey,
  //   stakingVaultBump: number,
  //   redeemableMint: anchor.web3.PublicKey,
  //   redeemableMintBump: Number;

  // it("Initialize Pool", async () => {
  //   [pool, poolBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(poolName)], program.programId);
  //   [stakingVault] = await anchor.web3.PublicKey.findProgramAddress(
  //     [Buffer.from("vault_seed"), Buffer.from(poolName)],
  //     program.programId
  //   );
  //   [poolAuthority, poolAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress(
  //     [Buffer.from("authority_seed"), Buffer.from(poolName)],
  //     program.programId
  //   );
  //   [redeemableMint, redeemableMintBump] = await anchor.web3.PublicKey.findProgramAddress(
  //     [Buffer.from("redeemable_mint"), Buffer.from(poolName)],
  //     program.programId
  //   );

  //   redeemableMintAcc = new Token(publicConnection, redeemableMint, TOKEN_PROGRAM_ID, payer);

  //   await program.rpc.initialize(poolName, {
  //     accounts: {
  //       payer: payer.publicKey,
  //       pool,
  //       stakingVault,
  //       stakingMint: wrappedSol,
  //       redeemableMint,
  //       authority: poolAuthority,
  //       systemProgram: SYSTEM_PROGRAM,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     },
  //     signers: [payer],
  //   });

  //   const poolState = await program.account.stakePool.fetch(pool);

  //   assert.ok(poolState.redeemableMint.equals(redeemableMint));
  //   assert.ok(poolState.stakingMint.equals(wrappedSol));
  //   assert.ok(poolState.stakingVault.equals(stakingVault));
  //   assert.ok(poolState.bumps.pool === poolBump);
  //   assert.ok(poolState.bumps.authority === poolAuthorityBump);
  //   assert.ok(poolState.bumps.stakingVault === stakingVaultBump);
  //   assert.ok(poolState.bumps.redeemableMint === redeemableMintBump);
  // });

  // it("Stake to the pool", async () => {
  //   userRedeemableToken = await Token.getAssociatedTokenAddress(
  //     ASSOCIATED_TOKEN_PROGRAM_ID,
  //     TOKEN_PROGRAM_ID,
  //     redeemableMint,
  //     user.publicKey
  //   );

  //   await program.rpc.stake(new anchor.BN(initialAmount * LAMPORTS_PER_SOL), {
  //     accounts: {
  //       pool,
  //       stakingVault,
  //       redeemableMint,
  //       source: userWrappedSol,
  //       destination: userRedeemableToken,
  //       poolAuthority,
  //       userAuthority: user.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     preInstructions: [
  //       Token.createAssociatedTokenAccountInstruction(
  //         ASSOCIATED_TOKEN_PROGRAM_ID,
  //         TOKEN_PROGRAM_ID,
  //         redeemableMint,
  //         userRedeemableToken,
  //         user.publicKey,
  //         user.publicKey
  //       ),
  //     ],
  //     signers: [user],
  //   });

  //   const userWrappedSolAcc = await wrappedSOLMintAcc.getAccountInfo(userWrappedSol);
  //   assert.ok(userWrappedSolAcc.amount.eq(new anchor.BN(0)));
  //   const stakingVaultAcc = await wrappedSOLMintAcc.getAccountInfo(stakingVault);
  //   assert.ok(stakingVaultAcc.amount.eq(new anchor.BN(initialAmount * LAMPORTS_PER_SOL)));
  //   const userRedeemableTokenAcc = await redeemableMintAcc.getAccountInfo(userRedeemableToken);
  //   assert.ok(userRedeemableTokenAcc.amount.eq(new anchor.BN(initialAmount * LAMPORTS_PER_SOL)));
  // });

  // const unstakeAmount = 0.2;

  // it("Unstake to the pool", async () => {
  //   await program.rpc.unstake(new anchor.BN(unstakeAmount * LAMPORTS_PER_SOL), {
  //     accounts: {
  //       pool,
  //       stakingVault,
  //       redeemableMint,
  //       source: userRedeemableToken,
  //       destination: userWrappedSol,
  //       poolAuthority,
  //       userAuthority: user.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user],
  //   });

  //   const userWrappedSolAcc = await wrappedSOLMintAcc.getAccountInfo(userWrappedSol);
  //   assert.ok(userWrappedSolAcc.amount.eq(new anchor.BN(unstakeAmount * LAMPORTS_PER_SOL)));
  //   const stakingVaultAcc = await wrappedSOLMintAcc.getAccountInfo(stakingVault);
  //   assert.ok(stakingVaultAcc.amount.eq(new anchor.BN((initialAmount - unstakeAmount) * LAMPORTS_PER_SOL)));
  //   const userRedeemableTokenAcc = await redeemableMintAcc.getAccountInfo(userRedeemableToken);
  //   assert.ok(userRedeemableTokenAcc.amount.eq(new anchor.BN((initialAmount - unstakeAmount) * LAMPORTS_PER_SOL)));
  // });
});
