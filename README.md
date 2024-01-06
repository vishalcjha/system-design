## Running locally

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Running locally
```bash
cargo watch -w ./src  -s  "trunk serve"
```

## Compiling for Release
```bash
trunk build --release
```

Will generate site package in dist/.