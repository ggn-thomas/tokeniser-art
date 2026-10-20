import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import * as borsh from "borsh"
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { 
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SYSVAR_RENT_PUBKEY,
    SystemProgram,
    TransactionInstruction,
    Transaction,
} from "@solana/web3.js"

const RPC_SOLANA ='https://api.devnet.solana.com/';
const PROGRAM_ID = new PublicKey("BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT");
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

const CreateNftArgsSchema = {
    struct: {
        nft_title: "string",
        nft_symbol: "string",
        nft_uri: "string",
    },
};

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(readFileSync(path, "utf-8")))
    );
}

function borshSerialize(schema: borsh.Schema, data: object): Buffer{
    return Buffer.from(borsh.serialize(schema, data));
}

async function main(){
    const connection = new Connection(RPC_SOLANA, "confirmed");
    const payer = createKeypairFromFile(`${homedir()}/.config/solana/id.json`);

    const mintKeypair = Keypair.generate();

    // Program Derived Address to store token metadata
    const metadataAddress = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"), 
            TOKEN_METADATA_PROGRAM_ID.toBuffer(), 
            mintKeypair.publicKey.toBuffer()
        ],
        TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const instructionData = borshSerialize(CreateNftArgsSchema, {
        nft_title: "42cursus",
        nft_symbol: "42",
        nft_uri: "https://gateway.irys.xyz/7EB9ihT1gNXN4HQiHtAmSxpG8rg6onsDQTxdHDzCvN8v",
    });

    // Program Derived Address for master edition account
    const editionAddress = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
            Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const associatedTokenAddress = await getAssociatedTokenAddress(
        mintKeypair.publicKey,
        payer.publicKey,
    );

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // mint
            { pubkey: payer.publicKey, isSigner: true, isWritable: false }, // mint_authority
            { pubkey: associatedTokenAddress, isSigner: false, isWritable: true }, // ATA
            { pubkey: metadataAddress, isSigner: false, isWritable: true }, // metadata
            { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // rent
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
            { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false }, // metadata program
            { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // ATA program
            { pubkey: editionAddress, isSigner: false, isWritable: true}, // master edition
        ],
        programId: PROGRAM_ID,
        data: instructionData,
    });

    const tx = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer, mintKeypair]);

    console.log(` Mint address: ${mintKeypair.publicKey}`);
    console.log(` Tx signature: ${tx}`);
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});