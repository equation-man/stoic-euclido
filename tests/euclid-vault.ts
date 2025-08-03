import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAccount, TOKEN_2022_PROGRAM_ID, getMint } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { EuclidVault } from "../target/types/euclid_vault";

describe("euclid-vault", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.euclidVault as Program<EuclidVault>;

  // test accounts
  const payer = anchor.web3.Keypair.generate()
  // represents the pool
  let vault: PublicKey = null
  let testTokenMint: PublicKey = null
  let user1StakeEntry: PublicKey = null
  let user1Ata: PublicKey = null
  let user1StakeAta: PublicKey = null
  let user2StakeEntry: PublicKey = null
  let user3StakeEntry: PbulicKey = null

  // derive program authority PDA
  let [vaultAuthority, vaultAuthBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vault_auth")],
      program.programId
  )
  console.log("The vaultAuthority is", vaultAuthority)

  it("[Token22] Create test vault pool with Token22 tokens!", async () => {
    // GETTING THE MINT OF A TOKEN ACCOUNT.
    //const tokenAccountPubkey = new PublicKey("6CXZg5TqNrApwkc8rA3jkPtmZUyTfHtf3cqLS7LiPSSu");
    //const tokenAccount = await getAccount(provider.connection, tokenAccountPubkey, undefined, TOKEN_PROGRAM_ID);
    const mintAddress = new PublicKey("FJY1SmEKLWtKCWwS7RvhT2RqBkn89AuomihLwf73X6ny");
    //const mintAddress = tokenAccount.mint.toBase58();
    const mintInfo = await getMint(provider.connection, mintAddress);
    // mintInfo.decimals, mintInfo.supply.toString(), mintInfo.mintAuthority?.toBase58(), mintInfo.freezeAuthority?.toBase58().
    console.log("Mint Decimals are: ", mintInfo.decimals);
    

    // Derive vault log account PDA
    const [vaultLogs, logsBump] = await PublicKey.findProgramAddress(
        [mintInfo.toBuffer(), Buffer.from("vault_state")],
        program.programId
    )
    vault = vaultLogs

    // Derive the token vault account to be initialized.
    const [tokenVault, vaultBump] = await PublicKey.findProgramAddress(
        [mintInfo.toBuffer(), vaultAuthority.toBuffer(), Buffer.from("vault_seed")],
        program.programId
    )
    lockerVault = tokenVault

    // Call the init_vault ix on program.
    

  })
});
