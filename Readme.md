# Learn Positional Notation

Learn to answer questions like: Is 21 greater than 12?


# Development

Setup development environment

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

Show the page in development server:

```bash
trunk serve
```

This Rebuilds the app whenever a change is detected and updates the local server.


Build/release the page to `./dist` directory:

```bash
trunk build --release
```


This is based on the template
```bash
cargo install cargo-generate
cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template
```


# TODO

cargo.toml:
trunk v0.21.4