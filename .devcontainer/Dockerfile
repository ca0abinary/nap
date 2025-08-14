FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update \
    && apt-get install -y \
      binutils-aarch64-linux-gnu binutils-x86-64-linux-gnu \
      gcc-aarch64-linux-gnu gcc-x86-64-linux-gnu \
      make nasm qemu-user-static \
    && rm -rf /var/lib/apt/lists/*
