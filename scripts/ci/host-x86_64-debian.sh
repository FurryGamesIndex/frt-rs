#!/usr/bin/env false

targets=(
    x86_64-unknown-linux-gnu
    aarch64-unknown-linux-gnu
    x86_64-pc-windows-gnu
)

declare -A bin_suffix
bin_suffix[x86_64-unknown-linux-gnu]=""
bin_suffix[aarch64-unknown-linux-gnu]=""
bin_suffix[x86_64-pc-windows-gnu]=".exe"
bin_suffix[x86_64-apple-darwin]=""
bin_suffix[aarch64-apple-darwin]=""

declare -A strip_cmd
strip_cmd[x86_64-unknown-linux-gnu]="strip"
strip_cmd[aarch64-unknown-linux-gnu]="aarch64-linux-gnu-strip"
strip_cmd[x86_64-pc-windows-gnu]="x86_64-w64-mingw32-strip"
strip_cmd[x86_64-apple-darwin]=""
strip_cmd[aarch64-apple-darwin]=""

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="gcc"
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-gcc"
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="x86_64-w64-mingw32-gcc"