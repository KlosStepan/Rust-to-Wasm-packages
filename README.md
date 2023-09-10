# Shared-worker-running-WebAssembly
We will show how to implement simple web worker via Rust using WebAssembly. Such worker then can be used seamlessly in Vanilla JavaScript web application/React.js web application when bundled as `npm package`.
<p align="center">
  <img src="misc/imgs/rustwasmjs.png" alt="k8s-aks-img-tr" style="width: 20%;" />
</p> 

## Wasm projects overview
There are 2 possibilities of using Rust for building web application using WebAssembly:
1. ~~`wholesome application` in Rust operating DOM via browser DOM API,~~
2. `modules` Wasm binaries in JavaScript API wrapper.  

We are going to explore option `2` since our goal is to write modularized middleware in Rust.  

## Wasm module project setup

First we prepare ourselves by installing Rust and Cargo.
```
curl https://sh.rustup.rs -sSf | sh
```
Then we install `wasm-pack` for creating modularized Wasm binaries.
```
cargo install wasm-pack
``` 
Once we are ready we go to `/modules` folder and instantiate new one. We find it in `/modules/pwnrusthellowasm`.
```
cargo new --lib pwnrusthellowasm
```
We programm our desired functionality within the module.  

Then we can compile project and with some additional [index.html](https://github.com/KlosStepan/Shared-worker-running-WebAssembly/blob/main/index.html) in `/` folder try out its functionality.
```
Shared-worker-running-WebAssembly/modules/pwnrusthellowasm> wasm-pack build --target web
Shared-worker-running-WebAssembly> python3 -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```

## Preparing as package for npm
Rust to Wasm compilation can be carried out to accomodate npm module format as well. To prepare, or expose your prepared package run following command in the appopriate module.
```
Shared-worker-running-WebAssembly/modules/pwnrusthellowasm> wasm-pack build --target bundler
Shared-worker-running-WebAssembly/modules/pwnrusthellowasm> cd pkg
Shared-worker-running-WebAssembly/modules/pwnrusthellowasm> npm publish
``` 
## Fundamentals of Rust to Wasm compilation
The process of compiling Rust code to Wasm is solved for us by the compiler(TODO). However, we need to understand steps and tools that are used or take place in this process.

Compilation process:  
1. blabla,  
2. blabla2,
3. finally js/ts wrapping.

Tools and steps:
- biding,
- loading,
- etc.