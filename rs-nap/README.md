# Rust rewrite

## Goals

### Feature parity

- [x] Maintain roughly the same binary size
- [x] Zero requirements other than the linux kernel
- [x] Increased readability over assembly
- [x] Shared code base for all architectures

### Architecture support

- [x] x86_64 implementation
- [x] aarch64 implementation

## Run on a native processor

```sh
# Build
cargo build --release
# Display file size
ls -lah ./target/release/rs-nap
# Run for 3 seconds
./target/release/rs-nap 3
```

## Run on an emulated processor

```sh
# Install cross compiler and emulation layer

# Debian-based distros
sudo apt -y install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu qemu-user-static
```

```sh
# Build
cargo build --target aarch64-unknown-linux-gnu --release
# Display file size
ls -lah target/aarch64-unknown-linux-gnu/release/rs-nap
# Run for 3 seconds
qemu-aarch64-static target/aarch64-unknown-linux-gnu/release/rs-nap 3
```
