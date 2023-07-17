import { PublicKey } from "@solana/web3.js";


/**
 * 
 * @param value a 64 bit integer
 * @returns a Uint8Array of the 64 bit integer in little endian format
 */
function numberBuffer(value: bigint): Uint8Array {
    const bytes = new Uint8Array(8);
    for (let i = 0; i < 8; i++) {
        bytes[i] = Number(value & BigInt(0xff));
        value = value >> BigInt(8);
    }
    return bytes;
}
export const INVOICE_SEED = Buffer.from("invoice");

export const getInvoicePda = async (invoiceId: number, program: PublicKey) => {
    const [invoicePda, _raffleBump] = await PublicKey.findProgramAddressSync(
        [INVOICE_SEED, numberBuffer(BigInt(invoiceId))],
        program
    );
    return invoicePda
}