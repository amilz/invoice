import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";
import { Invoice } from "../target/types/invoice";
import { AUTH_KEYPAIR } from "./helpers/keys";
import { getInvoicePda } from "./helpers/utils";

describe("invoice", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = await anchor.workspace.Invoice as Program<Invoice>;
  const { connection } = program.provider;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Airdrop to AUTH Wallet and finalize before starting tests
  beforeEach(async () => {
    let { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash('finalized');
    const airdropTx = await connection.requestAirdrop(AUTH_KEYPAIR.publicKey, LAMPORTS_PER_SOL * 1);
    await connection.confirmTransaction({
      signature: airdropTx,
      lastValidBlockHeight,
      blockhash
    }, 'finalized');
  });

  it("First invoice is created!", async () => {
    const invoicePda = await getInvoicePda(1, program.programId);
    const payer = Keypair.generate();
    const description = "TEST";
    const invoiceId = new anchor.BN(1);
    const tx = await program.methods.create(invoiceId, description)
      .accounts({
        authority: AUTH_KEYPAIR.publicKey,
        invoice: invoicePda,
        payer: payer.publicKey,
      })
      .signers([AUTH_KEYPAIR])
      .transaction();
    let { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash();
    tx.feePayer = AUTH_KEYPAIR.publicKey;
    tx.recentBlockhash = blockhash;
    tx.lastValidBlockHeight = lastValidBlockHeight;
    const txId = await anchor.web3.sendAndConfirmTransaction(connection, tx, [AUTH_KEYPAIR], { commitment: "finalized" });
    const invoice = await program.account.invoice.fetch(invoicePda);
    // Need to use toBase58 b/c anchor uses a different PublicKey class
    expect(invoice.payer.toBase58()).equal(payer.publicKey.toBase58());
    // TODO @Aaron Time permitting add more tests (e.g., can't recreate, etc.)
  });
  it("Adds an Item!", async () => {
    const invoicePda = await getInvoicePda(1, program.programId);
    const item = "Cool Item";
    const cost = new anchor.BN(100);
    const qty = (3);
    const invoiceId = new anchor.BN(1);
    const tx = await program.methods.addItem(invoiceId, {
        item, 
        cost, 
        qty
      })
      .accounts({
        authority: AUTH_KEYPAIR.publicKey,
        invoice: invoicePda,
      })
      .signers([AUTH_KEYPAIR])
      .transaction();
    let { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash();
    tx.feePayer = AUTH_KEYPAIR.publicKey;
    tx.recentBlockhash = blockhash;
    tx.lastValidBlockHeight = lastValidBlockHeight;
    const txId = await anchor.web3.sendAndConfirmTransaction(connection, tx, [AUTH_KEYPAIR], { commitment: "finalized" });
    const invoice = await program.account.invoice.fetch(invoicePda);
    expect(invoice.lineItems[0].item).equal(item);
    // TODO @Aaron Time permitting add more tests 
  });

});
