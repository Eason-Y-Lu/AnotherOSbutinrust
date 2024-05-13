# OS time baby

Made in rust with like, 2 crates

## Build

### With Docker on Linux

If you have Docker installed you can just do

> [!NOTE]
> If you didn't follow the docker installion instructions, you might need to run it as root.

```bash
./linuxDocker.sh
```

### From source

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
