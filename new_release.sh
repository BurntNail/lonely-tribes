#!/usr/bin/env sh

mkdir -p Releases/new-release
rm -rf Releases/new-release/*
cargo build --release

cp -r assets Releases/new-release/
cp -r config Releases/new-release/
cp -r target/release/lonely_tribes_binary Releases/new-release

rm -rf Releases/new-release/assets/data/*
