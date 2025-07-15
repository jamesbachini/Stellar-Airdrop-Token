# Stellar Airdrop Token

A hands on tutorial and implementation for building a SEP‑41 token on Stellar, distributing it to a curated list of recipients via a Merkle Tree.

https://jamesbachini.com/sep-41-token-airdrop-merkle/

**Key steps:**
1. Collect Stellar SDEX users to use as a recipient list.
2. Generate a Merkle Tree (root + proofs).
3. Deploy a SEP‑41 token contract with claim function.
4. Build a frontend UI allowing eligible users to claim tokens.

## 📦 Repository Structure

```
.
├── build-list/        # Script to collect SDEX users and generate recipient list
├── merkle/            # Script to build Merkle tree + proofs JSON
├── contracts/         # SEP‑41 token contract (Rust + OpenZeppelin)
├── frontend/          # Web UI for claiming tokens
└── README.md          # <-- You're here
````
---

## 1. Curate Airdrop Recipients

Collect a list of eligible Stellar addresses and amounts.

```bash
cd build-list
git clone https://github.com/jamesbachini/Stellar-Airdrop-Token.git
cargo run
````

* Outputs a JSON array like:

  ```json
  [
    { "address": "G…QVC7", "amount": 1000000000 },
    { "address": "G…PQRVJ", "amount": 1000000000 }
  ]
  ```

* You can adjust distribution logic (e.g., tiers or weighting) via `build-list/src/main.rs`.

---

## 2. Generate Merkle Tree

Create a Merkle root and proofs file.

```bash
cd merkle
cargo run -- ../build-list/sdex-traders.json proofs.json
```

* The Merkle root is printed to the console.
* `proofs.json` contains Merkle proofs for each recipient.

---

## 3. Deploy SEP‑41 Token Contract

Leverage OpenZeppelin's Merkle Distributor. Use the following snippet for the `claim` function:

```rust
pub fn claim(e: &Env, index: u32, receiver: Address, amount: i128, proof: Vec<BytesN<32>>) {
    receiver.require_auth();
    let data = Receiver { index, address: receiver.clone(), amount };
    Distributor::verify_and_set_claimed(e, data, proof);
    Base::mint(e, &receiver, amount);
}
```

To deploy on testnet via `stellar-cli`, include the Merkle root:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/airdrop_token.wasm \
  --env testnet \
  --args merkle_root:<YOUR_MERKLE_ROOT_IN_HEX>
```

---

## 4. Frontend Web UI

A React-based interface for users to claim tokens. Starts from Philip Liu's example.

1. Copy `.env.example` to `.env`:

   ```bash
   cp frontend/.env.example frontend/.env
   ```
2. Paste your `proofs.json` (minified) into the project.
3. Update `frontend/src/packages/contract.ts` with your contract address.

Run the UI:

```bash
cd frontend
npm install
npm run dev
```

👉 Open in browser. If your address is in the Merkle tree, you’ll see a **“Claim”** button.

---

## ⚠️ Security & Disclaimer

* This uses OpenZeppelin’s Rust SEP‑41 implementation, including an unaudited Merkle Distributor exercise caution on mainnet.
* Always verify Merkle root + proofs before deploying.
* Frontend only works for addresses in the provided tree.

---

## 📝 License

MIT

---

## 🔗 Related Resources

Developer Quick Start:
https://stellar.org/developers?utm_source=james-bachini&utm_medium=social&utm_campaign=lemonade-kol-developers-q2-25

Developer Docs:
https://developers.stellar.org/?utm_source=james-bachini&utm_medium=social&utm_campaign=lemonade-kol-dev-docs-q2-25

Dev Diaries:
https://stellar.org/?utm_source=james-bachini&utm_medium=social&utm_campaign=lemonade-kol-dev-diaries-q2-25

Flipside Challenges:
https://flipsidecrypto.xyz/earn/journey/stellar-onboarding?utm_source=james-bachini&ut[…]dium=social&utm_campaign=lemonade-kol-flipside-quests-q2-25

Stellar Main Site:
https://stellar.org/?utm_source=james-bachini&utm_medium=social&utm_campaign=lemonade-kol-general-q2-25

Meridian 2025:
https://meridian.stellar.org/register?utm_source=james-bachini&utm_medium=social&utm_campaign=lemonade-kol-meridian-2025-q2-25
