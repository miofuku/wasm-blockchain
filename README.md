# wasm-blockchain

## Create a new directory for your project
mkdir wasm-blockchain-demo
cd wasm-blockchain-demo

## Initialize a new Rust project
cargo new --lib wasm-lib
cd wasm-lib 

## Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

## Build the WebAssembly module
cd wasm-lib
wasm-pack build --target web 

## From the project root directory
python -m http.server 8000