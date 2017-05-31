#!/usr/bin/env bash
set -e

docker build -t rustfmt-build --rm=false -f Build.dockerfile .

docker run -it -v $(pwd)/src:/src rustfmt-build /bin/sh -c "cd rustfmt && cargo build --release && cd ../codeclimate-rustfmt && cargo build --release"

mkdir -p bin
cp src/rustfmt/target/release/rustfmt bin/
cp src/codeclimate-rustfmt/target/release/codeclimate-rustfmt bin/

