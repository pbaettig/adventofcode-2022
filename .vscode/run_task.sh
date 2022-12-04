#!/bin//bash
[[ -z "$1" ]] && exit

cd "$(dirname) ${1}"
cargo run