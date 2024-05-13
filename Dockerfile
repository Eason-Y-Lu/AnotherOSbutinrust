FROM debian:latest

RUN apt-get update && apt-get upgrade -y && apt-get install build-essential curl wget gcc qemu-utils qemu-system-x86 qemu-system-gui xauth git x11-xserver-utils -y && touch /root/.Xauthority && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete --default-toolchain nightly

RUN . "$HOME/.cargo/env" && rustup override set nightly && rustup target add x86_64-unknown-none && rustup component add rust-src && rustup component add llvm-tools

WORKDIR /

RUN git clone https://github.com/RdStudios9145/AnotherOSbutinrust.git

RUN . "$HOME/.cargo/env" && cd AnotherOSbutinrust && cargo build
