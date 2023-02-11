extern crate proc_macro;

use enum_index::prelude::*;

#[derive(EnumIndex)]
pub struct MyStruct {
    _populate_me: i32,
}

fn main() {
    println!("main.rs has run!");
}
