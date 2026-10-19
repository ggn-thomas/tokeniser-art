# Documentation

## Utilité du NFT

Quand un élève quitte l'école 42, son niveau dans le cursus est certifié directement sur la blockchain Solana. Ce certificat est :

- **portable** : il appartient au wallet de l'élève, qui peut le présenter partout sans dépendre de l'intranet 42 ;
- **authentique** : il est immuable et ne peut pas être dupliqué.


## Adresses (devnet)

| | |
|---|---|
| Program ID | `BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT` |
| Mint du NFT | `49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831` |
| Explorateur | [Solscan](https://solscan.io/token/49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831?cluster=devnet) |

## Arborescence

```
code/
  Cargo.toml
  src/
    lib.rs              point d'entrée du programme, appelle le processor
    processor.rs        désérialise les arguments, puis exécute create puis initialize_mint
    instructions/
      mod.rs            déclare les modules create et mint
      create.rs         crée le compte mint, l'initialise, puis crée le compte de métadonnées (non modifiables)
      mint.rs           crée le compte de tokens du payer, y mint 1 token, puis crée la Master Edition
deployment/
  deploy.sh             compile le programme et le déploie sur devnet
  mint/
    mint.ts             script client qui appelle le programme déployé pour minter un NFT
    package.json        dépendances du script
  public/
    nft-42.png          image du NFT
    metadata.json       métadonnées off-chain, stockées sur Irys
```

## Métadonnées

Les métadonnées d'un NFT Metaplex sont réparties en deux couches.

**On-chain** : le compte de métadonnées créé par le programme contient le nom, le symbole et l'URI. Ces valeurs sont passées en arguments de l'instruction.

**Off-chain** : le fichier `metadata.json`, stocké sur Irys, contient l'image, la description et les attributs. Irys permet de stocker de façon permanente des données trop lourdes pour la chaîne.


## Fonctionnement du programme

Le programme expose une seule instruction. Elle prend en argument `nft_title`, `nft_symbol` et `nft_uri` (sérialisés en Borsh) et exécute dans l'ordre :

1. création et initialisation du compte mint, avec 0 décimale ;
2. création du compte de métadonnées via un appel au programme Metaplex Token Metadata ;
3. création du compte de tokens associé (ATA) du payer ;
4. mint d'un token vers ce compte ;
5. création de la Master Edition avec `max_supply = 0`.

## Ownership et privilèges

| Rôle | Détenteur | Pouvoir |
|---|---|---|
| Propriétaire du NFT | le wallet qui détient le token dans son compte de tokens | peut transférer le NFT |
| Update authority | le wallet qui a minté | aucun, car `is_mutable = false` |
| Mint authority et freeze authority | la PDA Master Edition | aucune nouvelle émission n'est possible |

## Vérifier le propriétaire du NFT

Équivalent de `ownerOf` en Solidity. On cherche le compte de tokens qui détient le NFT, puis le propriétaire de ce compte :

```bash
# 1. Compte de tokens qui détient le NFT
curl -s https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
{"jsonrpc":"2.0","id":1,"method":"getTokenLargestAccounts","params":["49vhNmLw4R67yjPvwmdQyksqQXB2vDhjxWC3Xp9cj831"]}'

# 2. Propriétaire de ce compte (champ "owner")
curl -s https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
{"jsonrpc":"2.0","id":1,"method":"getAccountInfo","params":["<adresse obtenue à l étape 1>",{"encoding":"jsonParsed"}]}'
```

L'onglet « Holders » de Solscan affiche la même information.
