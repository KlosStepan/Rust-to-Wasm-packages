# Rust-to-Wasm-packages
We will describe how to implement simple web packages in Rust via WebAssembly. Such package then can be used seamlessly in `Vanilla JavaScript web application` or `React.js web application` compiled as `JavaScript modules` [^1] published at `npm` [^2].
<p align="center">
  <img src="misc/imgs/rustwasmjs.png" alt="rust-wasm-js" style="width: 20%;" />
</p> 

## Wasm project overview
There are 3 possibilities of using Rust for building web application targeting WebAssembly:
1. ~~`wholesome application` in Rust operating DOM via browser DOM API [^3],~~
2. `modules` that are Wasm binaries wrapped with JavaScript API [^4],
3. `React Component` using [wasm_react](https://docs.rs/wasm-react/latest/wasm_react/) for using React stuff in Rust.  

We are going to explore option `2` since our goal is to write modularized middleware in Rust.  

## Wasm module project setup

First we prepare ourselves by installing Rust and Cargo to Unix system.
```
curl https://sh.rustup.rs -sSf | sh
```
Then we install [`wasm-pack`](https://github.com/rustwasm/wasm-pack), amazing utility built by [`The Rust and WebAssembly Working Group`](https://rustwasm.github.io) for creating modularized Wasm binaries and other.
```
cargo install wasm-pack
``` 
Once we are ready we go to `/modules` folder and instantiate new one.  
We can use fiddle project in `/hellowasm-fiddle`.
```
cargo new --lib hellowasm-fiddle
```
We code our desired functionality in the Rust module.  

Then we can compile project and with some additional [index.html](https://github.com/KlosStepan/Shared-worker-running-WebAssembly/blob/main/index.html) in `/hellowasm-fiddle` folder try out its functionality.
```
Rust-to-Wasm-packages/hellowasm-fiddle> wasm-pack build --target web
Rust-to-Wasm-packages/hellowasm-fiddle> python3 -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```
We added [Makefile](https://github.com/KlosStepan/Rust-to-Wasm-packages/blob/main/hellowasm-fiddle/Makefile) for better dev experience, such as `build`, `run`, `clean`, etc.
## Distribute Rust module as `npm package`
Rust to Wasm compilation can be carried out to accomodate `npm module` format as well. We have to set target to `--target bundler`. To prepare, or expose our prepared packed functionality run following commands in the appopriate module folder. Then `package.json` with appropriate entrypoint to `pwnrusthellowasm.js` is generated for us.
```
Rust-to-Wasm-packages/modules/pwnrusthellowasm> wasm-pack build --target bundler
Rust-to-Wasm-packages/modules/pwnrusthellowasm> cd pkg
Rust-to-Wasm-packages/modules/pwnrusthellowasm/pkg> npm publish
``` 

## Rust to Wasm Overview
Rust[^6] is prepared to run in the web browser. There are several key takeways and tools making it possible.
- WebAssembly Core Specification (2019) [^5] bringing `Wasm` runtime to browser standard.
- `wasm-pack` tool for compiling code to WebAssembly.
- `wasm-bindgen` to communicate between Rust and JavaScript.

Then `Rust lib` -> `ES6 code` / `ES6 module`, `Cargo.toml` -> `package.json`, + `js wrapper`.  
Our final code, js wrapper and wasm is then found in `/pkg` directory.  

## Automated building
We have `Makefile` in each module to easily build and publish it to npm.
```bash
├── modules
|   ├── pwnqrpayment
│   |    ├── src/ & ...
│   |    └── Makefile
|   ├── pwnrusthellowasm
│   |    ├── src/ & ...
│   |    └── Makefile
|   └── pwn...
│        ├── ...
│        └── Makefile
├── index.html - basic Vanilla JS demonstration
└── README.md
```
We call `make` in order to build and public package to npm.  
We call `make clean` in order to remove `pkg` and `target`.  

Example of cleaning after successful build and publication.
```bash
Rust-to-Wasm-packages/modules/pwnrusthellowasm> make clean
# Remove the pkg and target directories if they exist
rm -rf pkg target
```

## Demonstration
We will demonstrate how to use all these packages[^7] in Next.js (React/Node) project [^8].  
## TODO List
- [ ] Read/study/implement https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
- [ ] Recursive Makefile calls.
- [ ] Makefiles for modules - unified one for build, publish run clean - standardize.
- [x] Read/study https://developer.mozilla.org/en-US/docs/WebAssembly/Loading_and_running
- ~~[ ] Make dummy 3rd package `crud` for Node.js backend - adding, modifying and removing companies using our services.~~
- [x] Prepare Nuxt.js application.
- [x] Test `hello` for correct behavior.
- [ ] Prepare `auth mock middleware` for frontend usage.
- [ ] Import `auth mock middleware` for frontend usage.
- [ ] Run `auth mock middleware` as `shared worker``.
- ~~[ ] Use dummy `crud` behind Node.js endpoints/services/functions/whatever after "login" to perform stuff on frontend.~~
- [x] Put live Next-ft-Wasm somewhere to Kubernetes Cluster.


## Notes
- Allowing Wasm packages in Nuxt.js config solution like https://github.com/vercel/next.js/blob/canary/examples/with-webassembly/next.config.js

[^1]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules
[^2]: https://www.npmjs.com/settings/pwnstepo/packages
[^3]: https://rustwasm.github.io/wasm-bindgen/examples/dom.html
[^4]: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
[^5]: https://www.w3.org/TR/wasm-core-1/
[^6]: https://www.rust-lang.org
[^7]: https://github.com/KlosStepan/Next-ft-Wasm
[^8]: http://wasm.stkl.cz