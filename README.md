# WebAssembly Blockchain Demo

This project demonstrates the integration of WebAssembly (Wasm) with blockchain functionality in a web browser. It includes a simple Rust library compiled to WebAssembly and basic MetaMask wallet integration.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python (for running a local server)
- A web browser with MetaMask installed


## Building and Running

1. **Build the WebAssembly Module**

   ```bash
   cd wasm-lib
   wasm-pack build --target web
   ```

   This will create a `pkg` directory containing the compiled WebAssembly code.

2. # Go back to root directory and create pkg directory
   ```bash
   cd ..
   mkdir -p pkg
   cp -r wasm-lib/pkg/* pkg/
   ```

3. **Start the Local Server**

   From the project **root** directory:
   ```bash
   python -m http.server 8000
   ```

4. **Access the Application**

   Open your web browser and navigate to:
   ```
   http://localhost:8000
   ```

## Features

- **WebAssembly Functions**
  - Text hashing (simple demonstration)
  - Number addition
- **Blockchain Integration**
  - MetaMask wallet connection

## Testing the Application

1. Enter text in the "Enter text to hash" input and click "Calculate Hash"
2. Try the number addition feature with two numbers
3. Click "Connect Wallet" to test MetaMask integration

## Troubleshooting

- If you see a "Please install MetaMask!" message, make sure you have the MetaMask browser extension installed
- If the WebAssembly functions don't work, check the browser console for errors and ensure the `pkg` directory was created correctly
