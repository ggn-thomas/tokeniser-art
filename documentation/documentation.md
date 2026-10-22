# Documentation

## Purpose of the NFT

When a student leaves 42, their level in the cursus is certified directly on the Solana blockchain. This certificate is:

- **portable**: it belongs to the student's wallet, so they can show it anywhere without relying on the 42 intranet;
- **authentic**: it is immutable and cannot be duplicated.


## Addresses (devnet)

| | |
|---|---|
| Program ID | `HgsUr9BvczUfQBxQd9df8GFXzdctxr15SwNyWysZSfbs` |
| NFT mint | `G5nrmND2Mhbbx15kD67GnBsD8XmhutuFwqWzUJ7tNQSs` |
| Explorer | [Solscan](https://solscan.io/token/G5nrmND2Mhbbx15kD67GnBsD8XmhutuFwqWzUJ7tNQSs?cluster=devnet) |

## File tree

```
code/
  Cargo.toml
  src/
    lib.rs              program entry point, calls the processor
    processor.rs        deserializes the arguments, then runs create and then initialize_mint
    instructions/
      mod.rs            declares the create and mint modules
      create.rs         creates the mint account, initializes it, then creates the metadata account (immutable)
      mint.rs           creates the payer's token account, mints 1 token to it, then creates the Master Edition
deployment/
  deploy.sh             builds the program and deploys it to devnet
  mint/
    mint.ts             client script that calls the deployed program to mint an NFT
    package.json        script dependencies
  public/
    nft-42.png          NFT image
    metadata.json       off-chain metadata, stored on Irys
```

## Metadata

The metadata of a Metaplex NFT is split into two layers.

**On-chain**: the metadata account created by the program holds the name, the symbol and the URI. These values are passed as instruction arguments.

**Off-chain**: the `metadata.json` file, stored on Irys, holds the image, the description and the attributes. Irys stores data that is too large for the chain, permanently.


## How the program works

The program exposes a single instruction. It takes `nft_title`, `nft_symbol` and `nft_uri` as arguments (Borsh-serialized) and runs, in order:

1. creation and initialization of the mint account, with 0 decimals;
2. creation of the metadata account, through a call to the Metaplex Token Metadata program;
3. creation of the payer's associated token account (ATA);
4. minting of one token to that account;
5. creation of the Master Edition with `max_supply = 0`.

## Ownership and privileges

| Role | Held by | Power |
|---|---|---|
| NFT owner | the wallet holding the token in its token account | can transfer the NFT |
| Update authority | the wallet that minted the NFT | none, because `is_mutable = false` |
| Mint authority and freeze authority | the Master Edition PDA | no new token can be issued |

## Checking who owns the NFT

This is the equivalent of `ownerOf` in Solidity. First find the token account that holds the NFT, then the owner of that account:

```bash
# 1. Token account holding the NFT
curl -s https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
{"jsonrpc":"2.0","id":1,"method":"getTokenLargestAccounts","params":["49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831"]}'

# 2. Owner of that account ("owner" field)
curl -s https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
{"jsonrpc":"2.0","id":1,"method":"getAccountInfo","params":["<address from step 1>",{"encoding":"jsonParsed"}]}'
```

The "Holders" tab on Solscan shows the same information.
