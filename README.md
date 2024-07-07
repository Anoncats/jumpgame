# Anoncat Jump Jump

Jump game for anoncat

## Development

### Native
You can simply run
```
cargo run
```

### Web
We need to compile to webassembly

```
rustup target add wasm32-unknown-unknown
```

Install `wasm-server-runner` this helps you to spawn a browser instance
```
cargo install wasm-server-runner
```


To run it on the browser simply run
```
cargo run --target wasm32-unknown-unknown
```

## Building For the Web

We need to compile to webassembly

```
rustup target add wasm32-unknown-unknown
```

Next we need the `wasm-bindgen-cli`
```
cargo install -f wasm-bindgen-cli
```

Build for wasm
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/ --target web ./target/wasm32-unknown-unknown/release/jumpgame.wasm
```

Serve the webapp
```
cd web
npx serve .
```