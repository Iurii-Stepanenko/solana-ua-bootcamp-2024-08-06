import { Keypair } from "@solana/web3.js";
import base58 from "bs58";

async function findPrivateKeyWithPrefix(prefix: string) {
    let attempts = 0;
    const startTime = Date.now();

    while (true) {
        attempts++;
        const keypair = Keypair.generate();
        const privateKeyBase58 = base58.encode(keypair.secretKey);

        if (privateKeyBase58.startsWith(prefix)) {
            const endTime = Date.now();
            console.log(`Found matching private key after ${attempts} attempts and ${(endTime - startTime) / 1000} seconds`);
            console.log(`Public Key: ${keypair.publicKey.toBase58()}`);
            console.log(`Private Key: ${privateKeyBase58}`);
            break;
        }
    }
}

const prefix = "anza";

findPrivateKeyWithPrefix(prefix);
