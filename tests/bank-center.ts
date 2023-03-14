import * as anchor from "@project-serum/anchor";
import {
  TOKEN_PROGRAM_ID,
  getAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import assert from "assert";
import { setup, createToken, mintTo } from "./setup";

describe("withdraw-testcase-token", async () => {
  const connection = new anchor.web3.Connection(
    "http://127.0.0.1:8899",
    "processed"
  );
  const path_authority_key = "/Users/minhdo/.config/solana/id.json";
  const authority = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(require(path_authority_key))
  ) as anchor.web3.Keypair;
  let mintAddress: anchor.web3.PublicKey;
  let escrowTokenAccount: anchor.web3.PublicKey;
  let configAccount: anchor.web3.PublicKey;
  let escrowVault: anchor.web3.PublicKey;
  let associatedAccount: anchor.web3.PublicKey;
  const { program } = await setup(connection, authority);

  before(async () => {
    mintAddress = await createToken(connection, authority);
    console.log({ mintAddress: mintAddress.toString() });
    /// Mint token to deposit wallet
    const amount = 1000;
    associatedAccount = await mintTo(
      connection,
      authority,
      authority,
      mintAddress,
      amount
    );
    console.log(associatedAccount.toString());
    escrowTokenAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow_pda_seed"),
        authority.publicKey.toBuffer(),
        mintAddress.toBuffer(),
      ],
      program.programId
    )[0];
    configAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("config_pda_seed"),
        authority.publicKey.toBuffer(),
        mintAddress.toBuffer(),
      ],
      program.programId
    )[0];

    escrowVault = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault_pda_seed"),
        authority.publicKey.toBuffer(),
        mintAddress.toBuffer(),
      ],
      program.programId
    )[0];
  });
  
  it("Init config", async () => {
    console.log(escrowVault.toBase58());
    const signature = await program.methods
      .initInstruction(new anchor.BN(10))
      .accounts({
        escrowTokenAccount: escrowTokenAccount,
        configAccount: configAccount,
        escrowVault: escrowVault,
        withdrawTokenAccount: associatedAccount,
        withdrawWallet: authority.publicKey,
        mintAddress: mintAddress,
        authority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("init: ", { signature });
  });

  it("Deposit token", async () => {
    const signature = await program.methods
      .depositInstruction(new anchor.BN(1000))
      .accounts({
        escrowTokenAccount: escrowTokenAccount,
        configAccount: configAccount,
        depositorTokenAccount: associatedAccount,
        depositor: authority.publicKey,
        mintAddress: mintAddress,
        authority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("deposit: ", { signature });
  });

  it("buy token", async () => {
    const signature = await program.methods
      .buyInstruction(new anchor.BN(10))
      .accounts({
        escrowTokenAccount: escrowTokenAccount,
        escrowVault: escrowVault,
        configAccount: configAccount,
        buyerTokenAccount: associatedAccount,
        buyer: authority.publicKey,
        mintAddress: mintAddress,
        authority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("buy: ", { signature });
  });
});
