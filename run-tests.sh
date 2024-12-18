#!/bin/bash
set -euo pipefail

# Runs a qemu test for a given architecture
# Ideally all builds will use the method below
run_qemu() {
  local arch=$1
  local seconds=$2
  local binary=$3
  local qemu="qemu-$arch-static"
  if [[ $binary == *"rs-"* ]]; then
    icon="🦀"
  else
    icon="🦬"
  fi

  echo "$icon $arch - Testing ${seconds}s nap"
  time timeout $(($seconds+1))s $qemu $binary $seconds &>/dev/null
}

## NASM tests - likely problems in the code somewhere

# NASM amd64 tests (does not work on aarch64 qemu)
if [ $(arch) == "x86_64" ]; then
  echo "🦬 x86_64 - Testing 1s nap"
  timeout 2s ./out/nap 1 &>/dev/null
  echo "🦬 x86_64 - Testing 10s nap"
  timeout 11s ./out/nap 10 &>/dev/null
fi

# NASM arm64 tests (does not work on x86_64 qemu)
if [ $(arch) == "aarch64" ]; then
  echo "🦬 aarch64 - Testing 1s nap"
  timeout 2s ./out/nap-aarch64 1 &>/dev/null
  echo "🦬 aarch64 - Testing 10s nap"
  timeout 11s ./out/nap-aarch64 10 &>/dev/null
fi

## Rust tests - rust works everywhere

# Rust amd64 tests
run_qemu x86_64 1 ./out/rs-nap-x86_64
run_qemu x86_64 10 ./out/rs-nap-x86_64

# Rust arm64 tests
run_qemu aarch64 1 ./out/rs-nap-aarch64
run_qemu aarch64 10 ./out/rs-nap-aarch64
