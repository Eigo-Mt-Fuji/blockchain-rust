# blockchain-rust

## Environments

|Name||Version|
|:----:|:----:|
|rust|1.62.1|

## How to run

- clippy lint

```shell
cargo clippy --fix
```

- run main

```rust
cargo run
```

## Note: install clippy to your mac

```shell
rustup update 
rustup component add clippy-preview
```

## Note: Install rust 1.62.1 to your mac

```shell
asdf plugin-add rust https://github.com/asdf-community/asdf-rust.git
asdf install rust 1.62.1
asdf global rust 1.62.1
asdf reshim rust
rustc --version # rustc 1.62.1 (e092d0b6b 2022-07-16)
cargo --version # cargo 1.62.1 (a748cf5a3 2022-06-08)
rustup --version # rustup 1.25.1
```
