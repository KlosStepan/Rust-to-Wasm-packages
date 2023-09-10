# Shared-worker-running-WebAssembly
We will show how to implement simple web worker via WebAssembly using Rust.

## Project fundamentals
There are 2 possibilities of using Rust for building WebAssembly web application:
1. `wholesome application` in Rust operating DOM via browser DOM API,
2. `modules` Wasm binaries in JavaScript wrapper.

## Project setup
We are going to delve into `2nd` option.  

First we prepare ourselves by installing Rust with Cargo.
```
curl https://sh.rustup.rs -sSf | sh
```
Then we install `wasm-pack` for creating modularized Wasm binaries.
```
cargo install wasm-pack
``` 
Once we are ready we go to `/modules` folder and instantiate new one.
```
cargo new --lib hello-wasm
```
## Compilation/Pipeline approach
TODO

## Further discussion
TODO