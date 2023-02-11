extern crate proc_macro;

use enum_index::prelude::*;

#[derive(Debug, EnumIndex)]
#[index_type(String)]
pub enum MyEnum {
    #[index("Member 0")]
    MEMBER0,
    #[index("Member 1")]
    MEMBER1,
}

fn main() {
    println!("{}", MyEnum::MEMBER0.index());
    println!("{:?}", MyEnum::try_from("Member 1"));
}
