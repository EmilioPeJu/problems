#!/bin/bash
EXEC="$(find -name '*.out' -printf '%T+ %p\n' | sort -r \
    | head -n 1 | cut -d ' ' -f 2)"
SRC_RS=${EXEC%.out}.rs
SRC_CPP=${EXEC%.out}.cpp
if [[ -f "$SRC_RS" ]]; then
    GDB="rust-gdb"
    ENTRY="_main"
else
    GDB="gdb"
    ENTRY="main"
fi
$GDB -ex "b $ENTRY" -ex 'r <in01'  makeproductequalone.out
