#!/bin/bash

set -e

cleanup() {
    if [ -d "build" ]; then
        rm -r build
    fi
}

trap cleanup EXIT

cleanup

if [ -f "./build.sh" ]; then
    chmod +x ./build.sh
else
    echo "build.sh не найден"
    exit 1
fi

/bin/bash ./build.sh

if [ -d "build" ]; then
    cd build
    if [ -f "./ecripus" ]; then
        chmod +x ./ecripus
        ./ecripus
    else
        echo "ecripus не найден"
        exit 1
    fi
    cd ../
else
    echo "Директория build не найдена"
    exit 1
fi
