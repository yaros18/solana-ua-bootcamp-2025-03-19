import "dotenv/config";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"));
console.log(`⚡️ Connected to devnet`);

const pubkey = new PublicKey("FvzhzFbbQf864XcMdFysPtL2s88FdUR32zdAy2iKXKrY");

//first
const airdrop1 = await airdropIfRequired(connection,pubkey,
    1 * LAMPORTS_PER_SOL,
    0.5 * LAMPORTS_PER_SOL
    );

    console.log('Airdrop1',airdrop1);

//second    
await connection.requestAirdrop(pubkey, 1 * LAMPORTS_PER_SOL);

const balanceInLamports = await connection.getBalance(pubkey);

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log("Balance in SOL",balanceInSOL);
