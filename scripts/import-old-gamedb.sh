#!/usr/bin/env bash

src="$1"
dst="$2"

[ -z "$dst" ] && errquit "usage: $0 src dst"

function copy_games {
    local src="$1"
    local src_a="$2"
    local dst="$3"

    mkdir -p "$dst"

    while read -r i; do
        name="${i%.yaml}"
        name="${name##*/}"

        echo "== $name"

        mkdir -p "$dst/$name"

        cp -v "$i" "$dst/$name/game.yaml"

        cp -v "$src_a/$name/"* "$dst/$name/"

    done < <(find "$src" -maxdepth 1 -type f -name '*.yaml')
}

copy_games "$src/games" "$src/assets" "$dst/games"