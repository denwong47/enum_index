///
///
extern crate proc_macro;

macro_rules! test_factory {
    (
        $name:ident
    ) => {
        #[cfg(test)]
        mod $name {
            extern crate proc_macro;
            use crate::prelude::*;

            #[derive(EnumIndex)]
            struct MyStruct {
                _populate_me: i32,
            }

            #[test]
            fn test_values() {
                assert_eq!(MyStruct { _populate_me: 0 }.test_enum_index(), 42069,)
            }
        }
    };
}

test_factory!(test_sample);
