<h4>Solana Auction Backend</h4>

A backend auction engine implemented as an on-chain program on Solana using Anchor.
This project translates a traditional Web2 auction backend into Solana’s account-based execution model, demonstrating how state management, permissions, and execution flow fundamentally change in a decentralized environment.

**Overview**

In a traditional backend:
-Auctions live in a database row
-Business logic runs on a trusted server
-Escrow is controlled by the application
-Admins can override or modify state

In this implementation:
-Auction state lives in program-owned accounts
-Business rules are enforced by the smart contract
-Funds are escrowed in a Program Derived Address (PDA)
-Settlement is deterministic and trustless
-There is no backend server. The program is the authority.

**Translating Web2 Backend → Solana Account Model**
Traditional Backend Concept → Solana Equivalent
-Database row → Auction account (PDA)
-Server-controlled escrow → Vault PDA (program-owned SOL account)
-Business logic layer → Anchor program instructions
-Access control middleware → Account constraints and signer checks
-Cron-based settlement → On-chain finalize_auction instruction
The design intentionally mirrors a classical backend auction engine while adapting it to Solana’s:
-Account ownership model
-Transaction atomicity
-Deterministic execution
-Lamport level fund control

**Core Components**
**Auction Account (PDA)**
Stores:
-Seller
-Start time
-End time
-Reserve price
-Highest bid
-Highest bidder
-Finalization status
This replaces a traditional auctions database table.

**Vault Account (PDA)**
-It's program-owned
-Holds escrowed SOL
-Refunds previous bidders
-Transfers final funds to seller
-This replaces the server managed escrow logic.

**Program Development**
Built using:
-Rust
-Anchor framework
-Solana native system program for SOL transfers

**Instructions**
*initialize_auction*
-Validates timing and reserve price
-Creates Auction account
-Prepares vault PDA

*place_bid*
-Enforces auction timing
-Validates the bid rules
-Transfers SOL to vault
-Refunds the previous highest bidder
-Updates the auction state

*finalize_auction*
-Validates auction has ended
-Transfers escrowed SOL to seller
-Marks auction as finalized
All logic is atomic and enforced at the program level.

**System Thinking: Web2 vs Web3 Backend**
This project demonstrates key architectural differences between traditional and on-chain backends.

**1. State Management**
Web2:
-Mutable database records
-Admin overrides are  possible

Solana:
-Explicit account structures
-Program enforced invariants
-No hidden state
-Deterministic updates

**2. Permissions & Authority**
Web2:
-Middleware checks
-Role-based access logic in server code

Solana:
-Signer validation
-Account ownership constraints
-PDA authority patterns
-Program-level enforcement
-Permissions are cryptographic, not session-based.

**3. Execution Model**
Web2:
-Server executes logic
-Can perform multi-step operations across requests

Solana:
-Execution occurs inside a single transaction
-Accounts are locked during execution
-State transitions must be atomic
-No background jobs
-Each instruction is a state transition function.

**4. Trust Model**
Web2:
Users trust the backend operator

Solana:
-Users trust the program logic
-Funds are escrowed transparently
-Settlement is deterministic
-The backend becomes verifiable infrastructure.

**Features**
English auction model
Native SOL bidding
On-chain escrow via PDA
Deterministic settlement
No anti-sniping extension
No off-chain authority

**Tech Stack**
-Rust
-Anchor
-Solana Local Validator (for development)

**Project Purpose**
This project is designed to:
-Demonstrate backend architectural translation from Web2 to Web3
-Showcase Solana program development using Anchor
-Highlight system-level thinking around state, permissions, and execution
-Serve as a foundation for more advanced marketplace logic
