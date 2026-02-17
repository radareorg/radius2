#!/bin/sh
vim radius/Cargo.toml
grep ^version radius/Cargo.toml | cut -d '"' -f2
