# Rust → Python RPC Client Generator

This repository provides a **procedural macro** for generating Python clients from Rust `tarpc` servers automatically. Using **PyO3** and **pyo3-asyncio**, it allows seamless Python integration with Rust RPC endpoints while supporting **all `serde`-serializable data types**.

In the future, this project aims to be able to generate `pyi` files in order to be able to provide type information and signatures of the generated packages for the python clients

---

## Features

* **Automatic client generation** for Python from Rust tarpc services
* Supports **async Python clients** using `pyo3-asyncio`
* Handles **all `serde`-serializable data types**, not just basic primitives
* Easy integration with existing Rust + tarpc projects
* Minimal boilerplate for Python-Rust RPC communication

---

## Installation

### Requirements

* **Python 3.8+**
* **Rust (with cargo)**
* **maturin** (for building Python extensions)

### Build Python Package

```bash
# Navigate to your Rust project folder
maturin develop -r
```

---

## Usage

1. **Define your Rust tarpc service**:

```rust
#[tarpc_python_client]
#[tarpc::service]
pub trait RpcAPI {
    async fn hellow(name: String) -> i32;
}
```

2. **Call the Python client**:

```python
from rpc_model import PyRpcAPIClient
import asyncio

async def main():
    stub = await PyRpcAPIClient.connect("127.0.0.1:5000")
    response = await stub.hello("Diego")
    print(response)


asyncio.run(main())
```

---

## Example

The `example/` folder contains a minimal working example demonstrating:

* RPC server implemented in Rust (`tarpc-server/`)
* Rust models compiled as Python extensions (`rpc-model/`)
* Python client connecting to the server (`client.py`)

Run the example by following the steps in `example/README.md`.

---

## How It Works

1. **Procedural Macro**: Generates Python bindings for the Rust tarpc service
2. **PyO3 Integration**: Exposes Rust functions and structs to Python
3. **Async Python Clients**: Uses `pyo3-asyncio` to handle async calls natively
4. **Serde Support**: Any Rust type that implements `Serialize`/`Deserialize` works seamlessly

---

## Project Structure

```
my-project/
│
├── macros/            # Procedural macro code
├── rpc-model/         # Rust models compiled as Python modules
├── tarpc-server/      # Rust tarpc server implementations
├── example/           # Example demonstrating Python client usage
└── client/            # Optional standalone Python clients
```

---

## Contributing

Contributions, bug reports, and feature requests are welcome! Please open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License.
