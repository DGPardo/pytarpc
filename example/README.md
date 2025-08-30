# Example Project — MVP

This folder contains a **minimum viable product (MVP)** demonstrating how to integrate a Python client with a Rust-based server using [tarpc](https://github.com/google/tarpc) and [maturin](https://github.com/PyO3/maturin).

---

## Quick Start

### 1. Create and activate a Python virtual environment
```bash
python -m venv .venv
source .venv/bin/activate   # On Linux / macOS
.venv\Scripts\activate      # On Windows (PowerShell)
```

### 2. Build the Python package (from Rust)
Navigate into the `rpc-model` folder and run:
```bash
maturin develop -r
```

### 3. Start the tarpc server
Navigate into the `tarpc-server` folder and run:
```bash
cargo run -r
```

### 4. Run the Python client
From the root of this folder (or wherever `client.py` is located), run:
```bash
python client.py
```

---

## Project Structure
```
example/
│
├── rpc-model/        # Rust code compiled into a Python package via maturin
├── tarpc-server/     # Rust server implementation using tarpc
└── client.py         # Python client for interacting with the server
```

---

## Requirements
- **Python 3.8+**  
- **Rust (with cargo)**  
- **maturin** (for building Python bindings)  
- **tarpc** (server library, installed automatically via cargo)

---

## How It Works
1. **The Rust model (`rpc-model/`)** is built as a Python extension module using maturin.  
2. **The tarpc server (`tarpc-server/`)** runs a Rust RPC service.  
3. **The Python client (`client.py`)** connects to the RPC server and demonstrates how to call Rust-backed functionality from Python.

