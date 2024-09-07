# Use the latest Ubuntu image as the base
FROM ubuntu:latest

# Install dependencies, including Python 3
RUN apt-get update && apt-get install -y \
    rustc \
    python3 \
    python3-pip \
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

# Install cargo-binutils and llvm-tools-preview
RUN cargo install cargo-binutils
RUN rustup component add llvm-tools-preview

# Add all embedded targets
RUN rustup update
RUN rustup target add thumbv6m-none-eabi
RUN rustup target add thumbv7m-none-eabi
RUN rustup target add thumbv7em-none-eabi
RUN rustup target add thumbv7em-none-eabihf
RUN rustup target add thumbv8m.base-none-eabi
RUN rustup target add thumbv8m.main-none-eabi
RUN rustup target add thumbv8m.main-none-eabihf
RUN rustup target add riscv32i-unknown-none-elf
RUN rustup target add riscv32imc-unknown-none-elf
RUN rustup target add riscv32imac-unknown-none-elf
RUN rustup target add riscv64gc-unknown-none-elf
RUN rustup target add riscv64imac-unknown-none-elf

ENTRYPOINT ["tail", "-f", "/dev/null"]