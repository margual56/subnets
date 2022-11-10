#!/bin/sh

CARGO_DIR=$HOME/.cargo/bin
targets=(x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-gnu aarch64-linux-android)
rm -rf ./bin
mkdir ./bin

for target in "${targets[@]}"
do
	echo Compiling $target
	$CARGO_DIR/cross build --target $target --release || exit
	mkdir ./bin/$target
	cp ./target/$target/release/subnets* ./bin/$target/
done

