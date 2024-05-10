# OS time baby

Made in rust with like, 2 crates

## Build

```bash
cargo build
```

but of course, you need to make sure you also

```
rustup override set nightly
rustup target install x86_64-unknown-none
rustup component add rust-src
rustup component add llvm-tools
```

first.
