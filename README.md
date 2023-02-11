# EnumIndex derive macro

![CI Checks](https://github.com/denwong47/enum_index/actions/workflows/CI.yml/badge.svg?branch=main)

This library provides a simple derive macro to add methods for bi-directional mapping between Enum variants and their respective values..

Example
-------

```rust
use enum_index::EnumIndex;


#[derive(EnumIndex)]
struct MyStruct {
    _populate_me: i32,
}

assert!(
    // Write a test here
    true
);

```
