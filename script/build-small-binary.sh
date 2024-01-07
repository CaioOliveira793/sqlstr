#!/usr/bin/env bash

set -e;

export CARGO_PROFILE_RELEASE_LTO='true';
export RUSTFLAGS='-C panic=abort -C codegen-units=1 -C strip=symbols';

cargo build --release --example small-binary;

bin_path='target/release/examples/small-binary';

echo -e "\nfile $bin_path:";
file $bin_path;

echo -e "\nstat $bin_path:";
stat $bin_path;

echo -e "\nldd $bin_path:";
ldd $bin_path;
