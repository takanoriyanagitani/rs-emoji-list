#!/bin/sh

WASM="./rs-emoji-list.wasm"

simple(){
    wasmtime run "${WASM}"
}

verbose(){
    wasmtime run "${WASM}" --verbose |
        jq -c
}

simple | tail -3 | file -
verbose | tail -3 | jq -c
