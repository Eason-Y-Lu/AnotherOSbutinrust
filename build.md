# Build

## With Docker on Linux

If you have Docker installed you can just do:

> [!NOTE]
> If you didn't follow the docker installion instructions, you might need to run the following scripts as root.

```bash
./linuxDocker.sh
```

You can stop the container with the following command:

```bash
./stop.sh
```

## From source

### Setup rust dependencies

```bash
rustup override set nightly
rustup target install x86_64-unknown-none
rustup component add rust-src
rustup component add llvm-tools
```

### Build

To build the entire project, run the following

```bash
cargo build
```

To run it using qemu (qemu must be installed), you can just run `cargo run`

If you only want to build the kernel without running it, you can still use `cargo build`. It will most likely print the location of the iso file for both UEFI and BIOS.

If there are any errors that occur doring the build, most likely that is due to some unknown dependencies that may not be present. Running commands that were printed in the build process will most likely help you.
