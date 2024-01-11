# Launching NFT Collection with Wallet Based Whitelist using Candy Machine V3

# Solana CLI
solana --version 
solana-cli 1.17.14 (src:cc7c59af; feat:1237720363, client:SolanaLabs)

solana-keygen --version
solana-keygen 1.17.14 (src:cc7c59af; feat:1237720363, client:SolanaLabs)

# Sugar
https://github.com/metaplex-foundation/sugar/releases/tag/v2.6.0

Copy to current Project DIR
 ./sugarV3
zsh: permission denied: ./sugarV3

chmod 755 sugarV3

./sugarV3

clear

# Check your current directory
pwd
/Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3

# Owner /Developer/ Received FUND Wallet Address: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/owner.json

# Creator Wallet Address: AP34FTLEJggrnAjApZB3V99G59Q3niVBDHjq55qyuY3Q
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/creator.json

# OG Wallet 1 Address: D1zbwtqesJg4j47BAqLZ4UEfqgLNmn8Rnbqz42uzCuqm
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/og_user1.json

# OG Wallet 2 Address: 64hXByruLythJg4L46eus47FxNHMzFBU2ntadRE9yPpr
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/og_user2.json


# Whitelist 1 Wallet Address: 3cnWESFiTamZSCgiCcdUmwHwkkro5CsVtSfEaJdu8Ba6
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/whitelist_user1.json


# Whitelist 2 Wallet Address: BwpFU4Ma6SQk6ZMpsBV2VuoPYkoQXtDQYg3idNobLDKv
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/whitelist_user2.json


# User Wallet Address: HPqjt4bmJCHankZFCGmMhksZbKdgBfEGSkisWWUPg1Hv
solana-keygen new --outfile /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/public_user1.json


# Setup Solana CLI

// Set wallet for Owner/Developer to develop
solana config set --keypair /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/owner.json

// Set metaplex devnet 
solana config set --url https://metaplex.devnet.rpcpool.com/

// Take a look to make sure all information is good
solana config get


RPC URL: https://metaplex.devnet.rpcpool.com/ 
WebSocket URL: wss://metaplex.devnet.rpcpool.com/ (computed)
Keypair Path: /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/owner.json
Commitment: confirmed


## Fund wallets

### Use this if airdrop does not work https://solfaucet.com/
### or Here https://faucet.quicknode.com/solana/devnet
// developer / owner Received FUND 
solana airdrop 1 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB  --url https://api.devnet.solana.com

Requesting airdrop of 1 SOL
Signature: 3QrFdzzUaJaqMihhiinjy1JBP6bbp4pUpyF7YpReiojrYXNLA5gaeCQYyMHT7nVw4FXmQRTXb8QwLC71kWzvomc4
1 SOL

// creator
solana airdrop 1 AP34FTLEJggrnAjApZB3V99G59Q3niVBDHjq55qyuY3Q --url https://api.devnet.solana.com

Requesting airdrop of 1 SOL
Signature: 56FJum6cmd7zcohQ4SEiPYL8Ra3UHMHY6wBaYDrXRisJtVksxsFvvVALUoLdiodn34Qwa6V4hX3oeBmpHAdLVWH6
1 SOL

// whitelist address
solana airdrop 1 3cnWESFiTamZSCgiCcdUmwHwkkro5CsVtSfEaJdu8Ba6  --url https://api.devnet.solana.com
Requesting airdrop of 1 SOL
Error: airdrop request failed. This can happen when the rate limit is reached.

# Go here to get more SOL https://solfaucet.com or Here https://faucet.quicknode.com/solana/devnet
// I just got 2 SOL here 
https://faucet.quicknode.com/solana/devnet/?transactionHash=3bNDQFYYdgQT4bSBxgbPasDZ7NRHY9d7H8H3thgnzxEchTSegrVJTZLw1BV57e7o7uj4nLrUejuG3b1EN3SB7zNQ

// public user
solana airdrop 1 HPqjt4bmJCHankZFCGmMhksZbKdgBfEGSkisWWUPg1Hv --url https://api.devnet.solana.com

Requesting airdrop of 1 SOL
Error: airdrop request failed. This can happen when the rate limit is reached.

// I have problem while request airdrop, so i decied to transfer 1 SOL from whitelist to Public address 
// that enough to cover fee on this demo.
solana transfer --keypair /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/whitelist_user1.json HPqjt4bmJCHankZFCGmMhksZbKdgBfEGSkisWWUPg1Hv 1 --url https://api.devnet.solana.com --allow-unfunded-recipient

// The --allow-unfunded-recipient flag in the Solana CLI transfer command is used to allow 
// the transfer of SOL to a recipient account that does not yet exist or has zero balance.


Signature: 39XP2fzkgcmdxepvzziEWy57gCcLtPEjHd8WJAq1ED3ThHZLacHkkQQSAnocowsWa9MbM2ZWAU1baEzZSq7K6gG

solana balance 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB --url https://api.devnet.solana.com
1 SOL

solana balance AP34FTLEJggrnAjApZB3V99G59Q3niVBDHjq55qyuY3Q --url https://api.devnet.solana.com 
1 SOL

solana balance 3cnWESFiTamZSCgiCcdUmwHwkkro5CsVtSfEaJdu8Ba6 --url https://api.devnet.solana.com
0.999995 SOL

solana balance HPqjt4bmJCHankZFCGmMhksZbKdgBfEGSkisWWUPg1Hv --url https://api.devnet.solana.com
1 SOL



# Send 0.5 SOL from OG 1 to OG 2
solana transfer --keypair /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/og_user1.json 64hXByruLythJg4L46eus47FxNHMzFBU2ntadRE9yPpr 0.5 --url https://api.devnet.solana.com --allow-unfunded-recipient

Signature: 2qjoPe5N5besj4cK5DeBakbNY1nDdQVCCNp1KfUtLDPxMELWwm3QBUdidRH577zNeE9tKHVA1jFY9efmUrme1X6b

# Send 0.5 SOL from OG 1 to Whitelist 2
solana transfer --keypair /Volumes/CodeSpace/mywebs/NFTs_MetaplexCandyV3/wallets/og_user1.json BwpFU4Ma6SQk6ZMpsBV2VuoPYkoQXtDQYg3idNobLDKv 0.5 --url https://api.devnet.solana.com --allow-unfunded-recipient

Signature: 5EWXJFtR6ULk6ZTx1vpsqs52XbU6UEhDWKTaBxffPpsVM6hFJrSmqVGZe2ZBGmHHrmFb6GEc33e6Jc8VLVELZcB9


# Download Metaplex Sample NFT Collection
https://docs.metaplex.com/assets/files/assets-ff6bd873ecd07b49c86faf3c7aab82d2.zip

Unzip to assets folder
Copy 'assets' folder to the current project folder

# Create Candy Machine V3 Config
./sugarV3 config create

1/2] 🍬 Sugar interactive config maker

Found metadata file(s) in folder 'assets':
  -> Loading values from file '0.json'

✔ Found 10 file pairs in "assets". Is this how many NFTs you will have in your candy machine? · yes
✔ Found symbol "NB" in your metadata file. Is this value correct? · yes
✔ What is the seller fee basis points? · 500
✔ Do you want to use a sequential mint index generation? We recommend you choose no. · no
✔ How many creator wallets do you have? (max limit of 4) · 1
✔ Enter creator wallet address #1 · AP34FTLEJggrnAjApZB3V99G59Q3niVBDHjq55qyuY3Q
✔ Enter royalty percentage share for creator #1 (e.g., 70). Total shares must add to 100. · 100
✔ Which extra features do you want to use? (use [SPACEBAR] to select options you want and hit [ENTER] when done) · 
✔ What upload method do you want to use? · Bundlr
✔ Do you want your NFTs to remain mutable? We HIGHLY recommend you choose yes. · yes

[2/2] 📝 Saving config file

Saving config to file: "config.json"

Successfully generated the config file. 🎉 

✅ Command successful.


# Upload Assets
./sugarV3 upload


[1/4] 🗂  Loading assets
Found 11 asset pair(s), uploading files:
+--------------------+
| images    |     11 |
| metadata  |     11 |
+--------------------+

[2/4] 🖥  Initializing upload

🛑 Error running command (re-run needed): HTTP status client error (403 Forbidden) for url (https://metaplex.devnet.rpcpool.com/)

# RPC metaplex devnet got error so i gonna change my RPC devnet
solana config set --url https://api.devnet.solana.com

Read More here https://docs.metaplex.com/resources/rpc-providers

./sugarV3 upload                                     

[1/4] 🗂  Loading assets
Found 11 asset pair(s), uploading files:
+--------------------+
| images    |     11 |
| metadata  |     11 |
+--------------------+

[2/4] 🖥  Initializing upload
▪▪▪▪▪ Connected
Funding address:
  -> pubkey: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
  -> lamports: 498751 (◎ 0.000498751)
Signature: 5VCdhZ3CZk4SE9zaS5eVVEx4zkkWgFgAY3dLW5ZVXAQ1cwZeM87vYE3chq5zS1FVDprceADPtMTX3Ebq36e8Xggx

[3/4] 📤 Uploading image files 

Sending data: (Ctrl+C to abort)
[00:00:01] Upload successful ████████████████████████████████████████████████████████████████████████████████████████████████ 11/11

[4/4] 📤 Uploading metadata files 

Sending data: (Ctrl+C to abort)
[00:00:00] Upload successful ████████████████████████████████████████████████████████████████████████████████████████████████ 11/11

11/11 asset pair(s) uploaded.

✅ Command successful.

# Deploy Candy Machine V3
# Candy machine ID: 4rYtJRQStiMYT2EG7Re5YtcH7tMc2npJN7DLpBwQevjT
./sugarV3 deploy


[1/3] 📦 Creating collection NFT for candy machine
Collection mint ID: WMauhay8qYto32Cit7Cg83nnbnLHwAE15QrTuL2Gf9k

[2/3] 🍬 Creating candy machine
Candy machine ID: 4rYtJRQStiMYT2EG7Re5YtcH7tMc2npJN7DLpBwQevjT

[3/3] 📝 Writing config lines
Sending config line(s) in 1 transaction(s): (Ctrl+C to abort)
[00:00:03] Write config lines successful ██████████████████████████████████████████████████████████████████████████████████████ 1/1

✅ Command successful.

# Verify Candy Machine V3
./sugarV3 verify


[1/2] 🍬 Loading candy machine
▪▪▪▪▪ Completed

[2/2] 📝 Verification
Verifying 10 config line(s): (Ctrl+C to abort)
[00:00:01] Config line verification successful ██████████████████████████████████████████████████████████████████████████████ 10/10

Verification successful. You're good to go!

See your candy machine at:
  -> https://www.solaneyes.com/address/4rYtJRQStiMYT2EG7Re5YtcH7tMc2npJN7DLpBwQevjT?cluster=devnet

✅ Command successful.


# Calculate Merkle Roots
cd merkel_rust and change my code to your OG wallet and WL wallet 
cargo run
# OG Merkle Root: 5e6d48e0f27c5e25bfcf17312bb32883e290d0486d471dcc7bd8169a7202756d
ecb91481165ad66846c8fa0905b44fcccaabb51473fbdb014e017fd961054bce
# OG Wallet 1 - D1zbwtqesJg4j47BAqLZ4UEfqgLNmn8Rnbqz42uzCuqm
# OG Wallet 2 - 64hXByruLythJg4L46eus47FxNHMzFBU2ntadRE9yPpr
# WL Merkle Root: 42627838aa35e16e1c9c2c67c1ee8deb8c35b5a926b3cc8967f3e0c4e058646e
cfe3866ffaaefed8f5838761e92b5dd756830d33189ac507e0ff7f1c4f00c785
# WL Wallet 1 - 3cnWESFiTamZSCgiCcdUmwHwkkro5CsVtSfEaJdu8Ba6
# WL Wallet 2 - BwpFU4Ma6SQk6ZMpsBV2VuoPYkoQXtDQYg3idNobLDKv

# Add Candy Guards to Candy Machine V3
./sugarV3 guard add

[1/3] 🔍 Looking up candy machine

Candy machine ID: 4rYtJRQStiMYT2EG7Re5YtcH7tMc2npJN7DLpBwQevjT

[2/3] 🛡  Initializing a candy guard
Signature: 3N8oEUtjg9h5Wy6VJ3azCyvEsTmqMdqcN7THmPEHDQ7mcBwQCaFozEALTsT5seYvGadYq4y35crnmCjCvn8TH88t

Candy guard ID: AoDRB94B3dqb1yRUBmgqKceovHXtyJGhaB6N2PP3kXyN

[3/3] 📦 Wrapping
Signature: 26f1htTGjkMoHovHLAAYA9gEUh1VgmXJ3Rzkbf9sCr7oxivbroLzKGjfZW3E8VJSo3yt9fPqph7BT3JHf6vMqkNY

The candy guard is now the mint authority of the candy machine.

✅ Command successful.

# Show Candy Guards added Candy Machine V3
./sugarV3 guard show
[1/1] 🔍 Loading candy guard
▪▪▪▪▪ Done

🛡  Candy Guard ID: AoDRB94B3dqb1yRUBmgqKceovHXtyJGhaB6N2PP3kXyN
 :
 :.. base: 8JQAorzD5qB8ZKLeGuX4WaKSPmAUkwbP7vjjzWvas8F2
 :.. bump: 255
 :.. authority: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
 :.. data: 
     :.. default: 
     :   :.. bot tax: none
     :   :.. sol payment: none
     :   :.. token payment: none
     :   :.. start date: none
     :   :.. third party signer: none
     :   :.. token gate: none
     :   :.. gatekeeper: none
     :   :.. end date: none
     :   :.. allow list: none
     :   :.. mint limit: none
     :   :.. nft payment: none
     :   :.. redeemed amount: none
     :   :.. address gate: none
     :   :.. nft gate: none
     :   :.. nft burn: none
     :   :.. token burn: none
     :   :.. freeze sol payment: none
     :   :.. freeze token payment: none
     :   :.. program gate: none
     :   :.. allocation: none
     :   :.. token2022 payment: none
     :
     :.. groups: 
          :.. label: OG
          :   :.. bot tax: 
          :   :   :.. lamports: 10000000 (◎ 0.01)
          :   :   :.. last instruction: true
          :   :.. sol payment: 
          :   :   :.. lamports: 100000000 (◎ 0.1)
          :   :   :.. destination: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
          :   :.. token payment: none
          :   :.. start date: 
          :   :   :.. date: Tue January  9 2024 09:00:00 UTC
          :   :.. third party signer: none
          :   :.. token gate: none
          :   :.. gatekeeper: none
          :   :.. end date: 
          :   :   :.. date: Wed March 27 2024 04:00:00 UTC
          :   :.. allow list: 
          :   :   :.. merkle root: 5e6d48e0f27c5e25bfcf17312bb32883e290d0486d471dcc7bd8169a7202756d
          :   :.. mint limit: 
          :   :   :.. id: 1
          :   :   :.. amount: 2
          :   :.. nft payment: none
          :   :.. redeemed amount: none
          :   :.. address gate: none
          :   :.. nft gate: none
          :   :.. nft burn: none
          :   :.. token burn: none
          :   :.. freeze sol payment: none
          :   :.. freeze token payment: none
          :   :.. program gate: none
          :   :.. allocation: none
          :   :.. token2022 payment: none
          :
          :.. label: WL
          :   :.. bot tax: 
          :   :   :.. lamports: 20000000 (◎ 0.02)
          :   :   :.. last instruction: true
          :   :.. sol payment: 
          :   :   :.. lamports: 200000000 (◎ 0.2)
          :   :   :.. destination: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
          :   :.. token payment: none
          :   :.. start date: 
          :   :   :.. date: Wed January 10 2024 19:00:00 UTC
          :   :.. third party signer: none
          :   :.. token gate: none
          :   :.. gatekeeper: none
          :   :.. end date: 
          :   :   :.. date: Wed March 27 2024 04:00:00 UTC
          :   :.. allow list: 
          :   :   :.. merkle root: 5e6d48e0f27c5e25bfcf17312bb32883e290d0486d471dcc7bd8169a7202756d
          :   :.. mint limit: 
          :   :   :.. id: 2
          :   :   :.. amount: 1
          :   :.. nft payment: none
          :   :.. redeemed amount: none
          :   :.. address gate: none
          :   :.. nft gate: none
          :   :.. nft burn: none
          :   :.. token burn: none
          :   :.. freeze sol payment: none
          :   :.. freeze token payment: none
          :   :.. program gate: none
          :   :.. allocation: none
          :   :.. token2022 payment: none
          :
          :.. label: public
              :.. bot tax: 
              :   :.. lamports: 30000000 (◎ 0.03)
              :   :.. last instruction: true
              :.. sol payment: 
              :   :.. lamports: 300000000 (◎ 0.3)
              :   :.. destination: 3HSt63DeavEkS7G3V8pr6e2cr9Sia1ovooRMzwG1RuaB
              :.. token payment: none
              :.. start date: 
              :   :.. date: Wed January 10 2024 19:00:00 UTC
              :.. third party signer: none
              :.. token gate: none
              :.. gatekeeper: none
              :.. end date: 
              :   :.. date: Wed March 27 2024 04:00:00 UTC
              :.. allow list: none
              :.. mint limit: 
              :   :.. id: 3
              :   :.. amount: 1
              :.. nft payment: none
              :.. redeemed amount: none
              :.. address gate: none
              :.. nft gate: none
              :.. nft burn: none
              :.. token burn: none
              :.. freeze sol payment: none
              :.. freeze token payment: none
              :.. program gate: none
              :.. allocation: none
              :.. token2022 payment: none

✅ Command successful.


# Update Candy Guards added Candy Machine V3
./sugarV3 guard update

chentony@192 NFTs_MetaplexCandyV3 % ./sugarV3 guard update
[1/2] 🔍 Loading candy guard
▪▪▪▪▪ Done
Candy guard ID: AoDRB94B3dqb1yRUBmgqKceovHXtyJGhaB6N2PP3kXyN

[2/2] 🖥  Updating configuration
Signature: 3PKnEbfqVpdAJUjsidoao8oYQbJntgcZzwvnoRq8LTYp5z48dh9ZwwYow4ntV9EPcLq7rGHMMwHyNfKaCzHFcs9K

# Setup Candy Machine V3 UI -  For Multi-group functionality use multi-group branch
# Download here: https://github.com/wearekite/candy-machine-v3-ui/tree/multi-group
# Or gh repo clone wearekite/candy-machine-v3-ui
unzip and copy to current project dir as name candy-machine-v3-ui
cd candy-machine-v3-ui 

# install node version >14.0.0 or nvm use  (not use too old node version) 
nvm use 20.9.0

# Install dependencies
yarn install

# change config file 
# DevNet
NEXT_PUBLIC_CANDY_MACHINE_ID=your_candy machine (devnet)
NEXT_PUBLIC_RPC_HOST=https://api.devnet.solana.com
NEXT_PUBLIC_SOLANA_NETWORK=devnet

# Mainnet
NEXT_PUBLIC_CANDY_MACHINE_ID=your_candy machine id (mainnet)
NEXT_PUBLIC_RPC_HOST=https://api.mainnet-beta.solana.com
NEXT_PUBLIC_SOLANA_NETWORK=mainnet


# Copy env file
rename examples.env to .env started server on 0.0.0.0:3000, url: http://localhost:3000

# Install dependencies
npm run dev // to run local
or cd candy-machine-v3-ui && npm run dev
# Run Key Stroke's Candy Machine V3 UI
# http://localhost:3000
npm run dev

