#!/bin/sh
vim radius/Cargo.toml
printf v
grep ^version radius/Cargo.toml | cut -d '"' -f2
