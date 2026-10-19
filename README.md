# TokenizeArt

Projet réalisé par **thgaugai**.

TokenizeArt crée un token non fongible (NFT) sur Solana. Le NFT certifie le niveau atteint par un élève dans le cursus 42.

## NFT déployé

| | |
|---|---|
| Blockchain | Solana |
| Réseau | Devnet |
| Program ID | `BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT` ([Solscan](https://solscan.io/account/BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT?cluster=devnet)) |
| Adresse du NFT (mint) | `49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831` ([Solscan](https://solscan.io/token/49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831?cluster=devnet)) |

## Choix techniques

**Blockchain : Solana.** Solana est l'une des blockchains les plus rapides du marché : elle traite des milliers de transactions par seconde pour un coût infime. Elle a une grande communauté de développeurs, donc beaucoup de ressources, et une adoption institutionnelle importante : Visa l'utilise pour des paiements en stablecoins et PayPal y a déployé son jeton PYUSD.

**Langage : Rust natif, sans framework.** Rust est le langage de référence des programmes Solana. Il est rapide et sa gestion stricte de la mémoire élimine une grande partie des failles de sécurité liées à la mémoire. Le programme n'utilise pas Anchor : chaque compte et chaque appel à un autre programme est écrit à la main, ce qui oblige à comprendre ce qui se passe réellement on-chain.

**Standard : Metaplex Token Metadata.** C'est le standard NFT de Solana, l'équivalent d'ERC-721 sur Ethereum. Il est reconnu par les wallets et les explorateurs.

**Stockage : Irys.** L'image et le fichier de métadonnées JSON sont stockés de façon permanente et décentralisée sur Irys. Seule l'URI du JSON est enregistrée on-chain, car l'espace on-chain est limité et payant.

**NFT immuable et unique.** Les métadonnées sont créées avec `is_mutable: false`, donc personne ne peut plus les modifier. La Master Edition est créée avec `max_supply: 0` et récupère la mint authority, donc aucun autre exemplaire ne peut être émis. Le mint authority appartient à la PDA Master Edition.


## Structure du dépôt

```
README.md
code/            programme Solana en Rust
deployment/      déploiement du programme et mint du NFT
documentation/   fonctionnement détaillé du NFT
```

## Utilisation

Prérequis : Rust, la [CLI Solana](https://solana.com/docs/intro/installation) configurée sur devnet avec un wallet approvisionné, et Node.js.

```bash
# Compiler et déployer le programme
./deployment/deploy.sh

# Installer les dépendances du script de mint
cd deployment/mint
npm install

# Minter un NFT
npx ts-node mint.ts
```

Le détail du fonctionnement, des comptes et des privilèges se trouve dans [documentation/documentation.md](documentation/documentation.md).
