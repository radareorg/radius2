#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function '^boolector_(.*)$' \
    --whitelist-type '^Btor(.*)$' \
    --whitelist-type '^Boolector(.*)$' \
    --blacklist-type '^BtorOpt(.*)$' \
    --no-recursive-whitelist \
    --raw-line 'use libc::FILE;' \
    --no-doc-comments \
    ../boolector/src/boolector.h

bindgen \
    --output options.rs \
    --generate types \
    --whitelist-type '^BtorOpt(.*)$' \
    --blacklist-type '^BtorOpt$' \
    --blacklist-type '^BtorOptHelp$' \
    --no-recursive-whitelist \
    --no-doc-comments \
    --no-prepend-enum-name \
    ../boolector/src/btoropt.h \
    -- \
    -I../boolector/src
