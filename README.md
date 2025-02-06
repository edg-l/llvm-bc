## LLVM bitcode writer
Allows using the a custom llvm builder api and writing llvm bitcode without using LLVM at compile time.

The goal is to make something like inkwell but more Rust idiomatic, since it doesn't have to deal with FFI at all.

Currently WIP.

Inspired heavily by https://github.com/indutny/bitcode
