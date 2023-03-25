#!/usr/bin/env bash

set -e

CI_DIR="$(realpath "$(dirname "${BASH_SOURCE[0]}")")"

source "$CI_DIR/host-x86_64-debian.sh"

rm -rf target/output
mkdir -p target/output

for i in "${targets[@]}"; do
    echo "=> $i"

    _bin_suffix="${bin_suffix[$i]}"
    #_strip="${strip_cmd[$i]}"

    cargo build \
        --profile relwithdbginfo \
        --target "$i"

    cp "target/$i/relwithdbginfo/frt$_bin_suffix" "target/output/frt-$i$_bin_suffix"

    #if [ -n "$_strip" ]; then
    #    cp "target/$i/relwithdbginfo/frt$_bin_suffix" "target/output/frt-stripped-$i$_bin_suffix"
    #    $_strip -K '*frt*' -w -s "target/output/frt-stripped-$i$_bin_suffix"
    #fi

    post_commands "$i" "target/output/frt-$i$_bin_suffix"

    gzip -f "target/output/frt-$i$_bin_suffix"
    #[ -f "target/output/frt-stripped-$i$_bin_suffix" ] && gzip -f "target/output/frt-stripped-$i$_bin_suffix"
done