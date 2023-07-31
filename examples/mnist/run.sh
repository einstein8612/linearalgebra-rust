#!/bin/bash

FILE=./mnist_train.csv
if ! test -f "$FILE"; then
    echo "Unzipping Mnist Training Data"
    unzip mnist_train.zip > /dev/null
fi

if [[ `rustc -V` != *"nightly"* ]]; then
  echo "You must run nightly Rust for this to work"
  exit 1
fi

cargo run --release