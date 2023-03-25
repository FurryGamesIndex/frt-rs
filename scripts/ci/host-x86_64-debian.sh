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

declare -A native_triple_prefix
native_triple_prefix[x86_64-unknown-linux-gnu]=""
native_triple_prefix[aarch64-unknown-linux-gnu]="aarch64-linux-gnu-"
native_triple_prefix[x86_64-pc-windows-gnu]="x86_64-w64-mingw32-"
#native_triple_prefix[x86_64-apple-darwin]=""
#native_triple_prefix[aarch64-apple-darwin]=""

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${native_triple_prefix[x86_64-unknown-linux-gnu]}gcc"
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="${native_triple_prefix[aarch64-unknown-linux-gnu]}gcc"
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${native_triple_prefix[x86_64-pc-windows-gnu]}gcc"

function post_commands {
    # Remove .debug_pubnames and .debug_pubtypes for GNU targets
    case "$1" in
    x86_64-unknown-linux-gnu|aarch64-unknown-linux-gnu|x86_64-pc-windows-gnu)
        "${native_triple_prefix[$1]}objcopy" -R .debug_pubnames -R .debug_pubtypes "$2"
        ;;
    esac
}