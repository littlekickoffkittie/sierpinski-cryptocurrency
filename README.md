
Built by https://www.blackbox.ai

---

# Sierpinski Cryptocurrency

## Project Overview
The Sierpinski Cryptocurrency project aims to create a unique blockchain-based cryptocurrency inspired by the mathematical properties of the Sierpinski triangle. The project explores complex geometrical concepts and implements a mining and transaction system based on fractal properties. Using Rust as the primary programming language, this project focuses on developing a robust and scalable cryptocurrency that leverages the beautiful structure of fractals.

## Installation
To get started with the Sierpinski Cryptocurrency project, you need to set up your development environment. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/sierpinski-cryptocurrency.git
cd sierpinski-cryptocurrency
cargo build
```

To run the project, execute the following command:

```bash
cargo run
```

## Usage
This project allows miners to participate in the Sierpinski triangle-based blockchain, enabling transactions that embed geometrical properties. To engage with the cryptocurrency:
- Start the node using `cargo run`.
- Use the command-line interface (CLI) to interact with your wallet and start mining.

### CLI Commands
You can use the following commands in the CLI:
- `mine`: Start the mining process.
- `create_wallet`: Generate a new Sierpinski triangle wallet.
- `send_triangle`: Send transactions between triangle wallets.

## Features
- **Fractal Hashing**: Utilizes a custom hashing algorithm that maintains properties of fractals.
- **Triangle Subdivision Mining**: Miners solve geometric puzzles related to triangle subdivisions.
- **Dynamic Difficulty**: Adjusts the mining difficulty based on network hash rates and triangle subdivision frequency.
- **Transaction Embedding**: Transactions are embedded within Sierpinski triangle coordinates.
- **Visual Blockchain Explorer**: A web-based tool to visualize the Sierpinski triangle blockchain in real-time.

## Dependencies
The project requires the following dependencies, as specified in `Cargo.toml`:

```toml
[dependencies]
sha3 = "0.10"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = "3.0"
```

## Project Structure
The project is organized into the following structure:

```
sierpinski-cryptocurrency/
├── src/
│   ├── crypto.rs                     # Cryptographic hash implementation
│   ├── mining.rs                     # Mining algorithms and configurations
│   ├── network.rs                    # Networking and P2P protocols
│   ├── transaction.rs                # Transaction management functionality
│   ├── wallet.rs                     # Wallet functionality and management
│   ├── triangle/
│   │   ├── geometry.rs               # Triangle geometrical calculations
│   │   ├── state.rs                  # State management for triangles
│   │   ├── validation.rs             # Validation checks for triangles
│   ├── cli.rs                        # Command-line interface implementation
│   └── visualization.rs              # Blockchain visualization tools
├── Cargo.toml                        # Cargo configuration file
├── README.md                         # Project documentation
└── TODO.md                           # TODOs and future enhancements
```

This structure will facilitate the rapid development and management of different components of the Sierpinski Cryptocurrency system. Each module focuses on a specific aspect of the system, promoting clarity and organization.

-----

For more detailed documentation and updates, refer to [the issue tracker](https://github.com/yourusername/sierpinski-cryptocurrency/issues) or consult the project's official documentation within the `/docs` directory (if applicable).