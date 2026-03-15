# 🧠 Memory Game – Soroban Smart Contract

## 📌 Project Description

Memory Game is a blockchain-powered project built using Soroban smart contracts on the Stellar network.
The purpose of this project is to demonstrate how decentralized applications (dApps) can store and manage game data securely on-chain.

This project was developed as part of a developer bootcamp task to learn smart contract development and blockchain integration with interactive applications.

---

## 🎮 What It Does

The Memory Game allows players to complete a card-matching game and submit their score to a smart contract.

The Soroban smart contract stores player scores and allows retrieving them from the blockchain. This makes the game data transparent, secure, and tamper-proof.

---

## ✨ Features

* Smart contract built using **Rust**
* Runs on the **Stellar blockchain**
* Stores player scores on-chain
* Supports multiple player submissions
* Retrieves all stored scores
* Calculates the best score (leaderboard concept)
* Beginner-friendly Soroban smart contract

---

## ⚙️ Tech Stack

* Soroban Smart Contracts
* Stellar Blockchain
* Rust Programming Language
* Stellar CLI

---

## 📂 Project Structure

```
memorygame-smart-contract
│
├── contracts
│   └── memorygame
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
│
├── README.md
└── .gitignore
```

---

## 🚀 How to Build the Contract

Install the Stellar CLI and build the contract:

```bash
stellar contract build
```

The compiled WebAssembly file (.wasm) will be generated in the target folder.

---

## 🌐 Deployed Smart Contract

Deployed Smart Contract Link:

memorygame

https://lab.stellar.org/r/testnet/contract/CBFCU7CGCWCRSR2T2OSNRWAHB275DHHVBSU4HP3O4GEQCLARS4EYMPQI

![Memory Game Screenshot](Screenshot 2026-03-15 at 12.56.18 PM.png)


---

## 🔮 Future Improvements

* Full leaderboard system
* Player profiles
* Token rewards for high scores
* Frontend integration with the memory game
* Multiplayer support

---

## 👨‍💻 Author

Built as part of a developer bootcamp project to explore Soroban smart contracts and blockchain application development.
