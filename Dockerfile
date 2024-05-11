FROM debian:latest

RUN apt-get update && apt-get upgrade -y && apt-get install curl wget gcc clang make qemu-utils qemu-system-x86 qemu-system-gui xauth build-essential git libgtk-4-dev x11-xserver-utils -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete --default-toolchain nightly

RUN . "$HOME/.cargo/env" && rustup override set nightly && rustup target add x86_64-unknown-none && rustup component add rust-src && rustup component add llvm-tools

WORKDIR /

RUN git clone https://github.com/RdStudios9145/AnotherOSbutinrust.git

RUN . "$HOME/.cargo/env" && cd AnotherOSbutinrust && cargo build
