# Use the latest Ubuntu image as the base
FROM ubuntu:latest

# Install dependencies, including Python 3
RUN apt-get update && apt-get install -y \
    rustc \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    git \
    gcc-avr \
    binutils-avr \
    avr-libc \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y \
    llvm \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


# Rust
RUN rustup default stable

# Install dependencies
RUN cargo install \
    cargo-binutils \
    cargo-play

RUN rustup component add \
    llvm-tools-preview \
    rust-src \
    rustfmt

# Add all embedded targets
RUN rustup update
RUN rustup target add \
    thumbv6m-none-eabi \
    thumbv7m-none-eabi \
    thumbv7em-none-eabi \
    thumbv7em-none-eabihf \
    thumbv8m.base-none-eabi \
    thumbv8m.main-none-eabi \
    thumbv8m.main-none-eabihf \
    riscv32i-unknown-none-elf \
    riscv32imc-unknown-none-elf \
    riscv32imac-unknown-none-elf \
    riscv64gc-unknown-none-elf \
    riscv64imac-unknown-none-elf


RUN mkdir /orbit

WORKDIR /orbit

ENTRYPOINT ["tail", "-f", "/dev/null"]