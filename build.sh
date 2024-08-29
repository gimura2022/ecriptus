#!/bin/bash

set -e

if [ ! -d "./build" ]; then
    cmake -S . -B ./build
fi

cd build

if [ -f "Makefile" ]; then
    make
else
    echo "Makefile не найден"
    exit 1
fi

cd ../
