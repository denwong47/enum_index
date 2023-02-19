///
///
extern crate proc_macro;

macro_rules! test_factory {
    (
        $name:ident,
        $type:ty,
        $bad_index:expr,
        $(($variant:ident, $index:expr),)+
    ) => {
        #[cfg(test)]
        mod $name {
            extern crate proc_macro;
            use crate::prelude::*;

            #[allow(dead_code)]
            type OptionalChar = Option<char>;

            #[derive(Debug, EnumIndex, PartialEq)]
            #[index_type($type)]
            enum TestEnum{
                $(
                #[index($index)]
                $variant
                ),*
            }

            #[test]
            fn test_indexs() {
                $(
                // Assert the index is correct
                assert_eq!(TestEnum::$variant.index(), $index);

                // Assert that we can get back the member from the index
                assert_eq!(TestEnum::from_index(&$index), Some(TestEnum::$variant));

                // Assert From/Into works
                assert_eq!(Into::<$type>::into(TestEnum::$variant), $index);

                // Assert TryFrom works
                assert_eq!(TestEnum::try_from(&$index).ok(), Some(TestEnum::$variant));

                // Assert VariantByName works
                assert_eq!(TestEnum::by_name(stringify!($variant)), Some(TestEnum::$variant));
                )*

                // Assert whatever not found is not foun
                assert!(TestEnum::try_from(&$bad_index).is_err());

                // Assert bad variant name as [`None`]
                assert!(TestEnum::by_name("").is_none());
            }
        }
    };
}

test_factory!(
    test_usize,
    usize,
    0,
    (Member00, 21089),
    (Member01, 37669),
    (Member02, 44970),
    (Member03, 13744),
    (Member04, 19764),
    (Member05, 90),
    (Member06, 21369),
    (Member07, 37779),
    (Member08, 50236),
    (Member09, 62233),
    (Member10, 39819),
    (Member11, 22817),
    (Member12, 16094),
    (Member13, 6853),
    (Member14, 12822),
    (Member15, 47341),
    (Member16, 54052),
    (Member17, 18324),
    (Member18, 20489),
    (Member19, 2374),
);

test_factory!(
    test_char,
    char,
    'C',
    (Member00, 'A'),
    (Member01, 'B'),
    (Member02, 'a'),
    (Member03, 'b'),
    (Member04, '0'),
    (Member05, '1'),
    (Member06, '\0'),
    (Member07, '\n'),
    (Member08, '\t'),
    (Member09, '\u{2591}'),
    (Member10, '\u{2503}'),
);

test_factory!(
    test_i16,
    i16,
    0,
    (Member00, 29192),
    (Member01, -5594),
    (Member02, -15365),
    (Member03, -15845),
    (Member04, -12846),
    (Member05, -32737),
    (Member06, 10587),
    (Member07, 27576),
    (Member08, -19107),
    (Member09, -29853),
    (Member10, 1079),
    (Member11, 7554),
    (Member12, 26404),
    (Member13, 21728),
    (Member14, 6922),
    (Member15, -4517),
    (Member16, -32440),
    (Member17, 26139),
    (Member18, 23869),
    (Member19, -1869),
);

test_factory!(
    test_optional_char,
    OptionalChar,
    Some('b'),
    (Member00, Some('a')),
    (Member01, None),
);

macro_rules! test_string_factory {
    (
        $name:ident,
        $type:ty,
        $bad_index:expr,
        $(($variant:ident, $index:expr),)+
    ) => {
        // #[cfg(test)]
        mod $name {
            extern crate proc_macro;
            use crate::prelude::*;

            #[derive(Debug, EnumIndex, PartialEq)]
            #[index_type($type)]
            enum TestEnum{
                $(
                #[index($index)]
                $variant
                ),*
            }

            #[test]
            fn test_indexs() {
                $(
                // Assert the index is correct
                assert_eq!(TestEnum::$variant.index(), $index.to_string());

                // Assert that we can get back the member from the index
                assert_eq!(TestEnum::from_index($index), Some(TestEnum::$variant));

                // Assert From/Into works
                assert_eq!(Into::<$type>::into(TestEnum::$variant), $index.to_string());

                // Assert TryFrom works
                assert_eq!(TestEnum::try_from($index).ok(), Some(TestEnum::$variant));

                // Assert VariantByName works
                assert_eq!(TestEnum::by_name(stringify!($variant)), Some(TestEnum::$variant));
                )*

                // Assert whatever not found is not found
                assert!(TestEnum::try_from($bad_index).is_err());

                // Assert bad variant name as [`None`]
                assert!(TestEnum::by_name("").is_none());
            }
        }
    };
}

test_string_factory!(
    test_string,
    String,
    "Invalid String",
    (Member00, "+rGkAwYWJvQ6HzhtXrCHs2A=="),
    (Member01, "B16yA7t01JYC2Um=="),
    (Member02, "+/j53ZTS+D58uzSBkZ4Jx3h/I+rDJPOFiYpgQ=="),
    (Member03, "p1KTZamYtV/=="),
    (Member04, "+/=="),
    (Member05, "+ZWqvT+UWSl+RwGw32KADTJHMlR6JcAGwE9YzvSmXw=="),
    (Member06, "=="),
    (Member07, "qAUg/+CRNaFatB/+p5fbb9vaHWKx5L5AG/immusSe3A=="),
    (Member08, "coNKPHaueMGmugP51el9QOkg/8hdPCeLoI/=="),
    (Member09, "H/hY9//+MrA=="),
    (Member10, "/OdtN4pEnRS5R3E18eiOklrZw=="),
    (Member11, "NmSYN+NUXDK/uRLhf2/URjjsGIxO8o/+=="),
    (Member12, "5ldEBs1mnW7SiyEKSp1IMvuroDJ/+KVBuR2QA=="),
    (Member13, "/zDjPbh5pmPtKphUKP2Bx/=="),
    (Member14, "xuSVukmyVvFPCWvLr/+=="),
    (Member15, "h+XsWI97Dnk5UMv3EsljAQsNPEQVsw=="),
    (Member16, "gb7Ts5feZnArxQh053h7Q=="),
    (Member17, "HIXOKV9aA=="),
    (Member18, "rgU79h6+jzLlrxMDkg=="),
    (Member19, "VWFeecoRdYVPmzUX1g=="),
);
