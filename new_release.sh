#!/usr/bin/env sh

mkdir Releases/new-release
rm -rf Releases/new-release/*
cargo build --release

cp -r assets Releases/new-release/
cp -r config Releases/new-release/
cp -r target/release/lonely-tribes Releases/new-release

rm -rf Releases/new-release/assets/data/*
