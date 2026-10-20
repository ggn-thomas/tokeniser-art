# TokenizeArt

Project by **thgaugai**.

TokenizeArt creates a non-fungible token (NFT) on Solana. The NFT certifies the level a student has reached in the 42 cursus.

## Deployed NFT

| | |
|---|---|
| Blockchain | Solana |
| Network | Devnet |
| Program ID | `BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT` ([Solscan](https://solscan.io/account/BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT?cluster=devnet)) |
| NFT address (mint) | `49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831` ([Solscan](https://solscan.io/token/49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831?cluster=devnet)) |

## Technical choices

**Blockchain: Solana.** Solana is one of the fastest blockchains available: it processes thousands of transactions per second at a negligible cost. It has a large developer community, which means plenty of resources, and significant institutional adoption: Visa uses it for stablecoin payments and PayPal has deployed its PYUSD token on it.

**Language: native Rust, no framework.** Rust is the reference language for Solana programs. It is fast, and its strict memory management rules out a large class of memory-related security flaws. The program does not use Anchor: every account and every call to another program is written by hand, which requires understanding what actually happens on-chain.

**Standard: Metaplex Token Metadata.** This is the NFT standard on Solana, the equivalent of ERC-721 on Ethereum. Wallets and explorers recognize it.

**Storage: Irys.** The image and the JSON metadata file are stored permanently and in a decentralized way on Irys. Only the URI of the JSON file is stored on-chain, because on-chain space is limited and costs rent.

**Immutable and unique NFT.** The metadata is created with `is_mutable: false`, so no one can modify it anymore. The Master Edition is created with `max_supply: 0` and takes over the mint authority, so no other copy can ever be issued. The mint authority belongs to the Master Edition PDA.

## Repository structure

```
README.md
code/            Solana program in Rust
deployment/      program deployment and NFT minting
documentation/   detailed explanation of how the NFT works
```

## Usage

Requirements: Rust, the [Solana CLI](https://solana.com/docs/intro/installation) configured for devnet with a funded wallet, and Node.js.

```bash
# Build and deploy the program
./deployment/deploy.sh

# Install the mint script dependencies
cd deployment/mint
npm install

# Mint an NFT
npx ts-node mint.ts
```

Details about how the program works, its accounts and its privileges are in [documentation/documentation.md](documentation/documentation.md).
