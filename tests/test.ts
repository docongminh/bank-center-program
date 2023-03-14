import * as anchor from "@project-serum/anchor";
import {
  TOKEN_PROGRAM_ID,
  getAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { BankCenter } from "../target/types/bank_center";

(async () => {
  const connection = new anchor.web3.Connection(
    "http://127.0.0.1:8899",
    "processed"
  );
  const idl = require("../target/idl/bank_center.json");
  const path_authority_key = "/Users/minhdo/.config/solana/id.json";
  const authority = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(require(path_authority_key))
  ) as anchor.web3.Keypair;
  const wallet = new anchor.Wallet(authority);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "processed",
  });

  const Program_ID = new anchor.web3.PublicKey(
    "3iUNmf8zTvnmFTCQyVpo3Kthz8Q1L7uPRvnskijdCJF2"
  );
  const program = new anchor.Program(
    idl,
    Program_ID,
    provider
  ) as Program<BankCenter>;
  const mintAddress = new anchor.web3.PublicKey(
    "AnZDosUoLMzUvM222wCQbhDcrgpdosC7eE1G2mwUooG5"
  );
  const associatedAccount = new anchor.web3.PublicKey(
    "4JpVAHT4NfMmQpNNs8YA1yz7ji69HvrKVXe6t5YtWCzy"
  );
  const escrowTokenAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow_pda_seed"),
      authority.publicKey.toBuffer(),
      mintAddress.toBuffer(),
    ],
    program.programId
  )[0];
  const configAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("config_pda_seed"),
      authority.publicKey.toBuffer(),
      mintAddress.toBuffer(),
    ],
    program.programId
  )[0];

  const escrowVault = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_pda_seed"),
      authority.publicKey.toBuffer(),
      mintAddress.toBuffer(),
    ],
    program.programId
  )[0];

  // const signature1 = await program.methods
  //   .initInstruction(new anchor.BN(10))
  //   .accounts({
  //     escrowTokenAccount: escrowTokenAccount,
  //     configAccount: configAccount,
  //     escrowVault: escrowVault,
  //     withdrawTokenAccount: associatedAccount,
  //     withdrawWallet: authority.publicKey,
  //     mintAddress: mintAddress,
  //     authority: authority.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //   })
  //   .rpc();
  // console.log("init: ", { signature1 });

  // const updateSig = await program.methods
  //   .updateConfigInstruction(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
  //   .accounts({
  //     configAccount: configAccount,
  //     mintAddress: mintAddress,
  //     authority: authority.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc();
  // console.log("init: ", { updateSig });

  // const signature2 = await program.methods
  //   .depositInstruction(new anchor.BN(1000))
  //   .accounts({
  //     escrowTokenAccount: escrowTokenAccount,
  //     configAccount: configAccount,
  //     depositorTokenAccount: associatedAccount,
  //     depositor: authority.publicKey,
  //     mintAddress: mintAddress,
  //     authority: authority.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .rpc();
  // console.log("deposit: ", { signature2 });

  const signature = await program.methods
    .buyInstruction(new anchor.BN(2))
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
})();
