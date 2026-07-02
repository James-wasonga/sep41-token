# SibToken (SEP-41 Soroban Token)

SibToken is a custom token built on the Stellar Soroban smart contract platform.  
It follows the SEP-41 token standard and implements core token functionalities such as minting, burning, transfers, allowances, and delegated transfers.

---
## 📦 Deployed Contract

**Contract Address:**  
`CCBQOYJ3HAAWHQ5HHQAV3C6V6HJKPOEZWFANZ5BVTLAWHZL3YYPB7FZO`

---

This repository uses the recommended structure for a Soroban project:

```text
.
.
├── contracts
│   └── sep41-token
│       ├── src
│       │   ├── lib.rs
│       │   ├── our_token.rs
│       │   ├── storage.rs
│       │   ├── events.rs
│       │   ├── error.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## 📌 Overview

This project demonstrates a fully functional smart contract token written in Rust using the Soroban SDK. It includes:

- Token initialization with an admin and initial supply
- Balance tracking
- Allowance-based approvals
- Secure transfers and delegated transfers
- Minting and burning functionality
- Event emissions for key state changes

---

## ⚙️ Features

### 🔐 Access Control
- Admin-based minting
- Authorization checks using `require_auth()`

### 💰 Token Operations
- `transfer()` — Send tokens between accounts
- `transfer_from()` — Delegated transfers using allowance
- `approve()` — Approve spender allowance
- `burn()` — Destroy tokens from own balance
- `burn_from()` — Burn tokens using allowance
- `mint()` — Create new tokens (admin only)

### 📊 Token Metadata
- `name()` → `"SibToken"`
- `symbol()` → `"SIB"`
- `decimals()` → `18`

