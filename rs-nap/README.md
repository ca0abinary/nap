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

#### Architecture specific implementation notes

For each new architecture:

1. Add a file to the `support` subfolder which implements
   - The startup code (`_start`) to get arguments from the stack
   - and the following linux kernel service calls
     - `sys_exit` to call `exit`
     - `sys_write` to call `write` to stdout 1
     - and `sys_sleep`
2. Add a platform specific entry to `support.rs` in the form of a `cfg_attr`
3. Add a platform target to `.cargo/config.toml`
4. Add a toolchain target to `rust-toolchain.toml`
5. Update this `README.md` with the new platform build and run commands
6. Update any CI/CD (future) to build the binary for the target platform

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
