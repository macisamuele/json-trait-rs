#[macro_export]
macro_rules! rust_type {
    // Disclaimer: This TT muncher is a slightly modified version of the json! macro
    // provied by the serde-json crate (https://github.com/serde-rs/).
    // Thanks to @dtolnay
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: rust_type!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        rust_type!(@array [$($elems,)* rust_type!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        rust_type!(@array [$($elems,)* rust_type!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        rust_type!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        compile_error!(concat!("unexpected", stringify!($unexpected)));
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: rust_type!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        rust_type!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        compile_error!(concat!("unexpected", stringify!($unexpected)));
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        rust_type!(@object $object [$($key)+] (rust_type!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        compile_error!("Unexpected end of the json Object (missing the value)")
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        compile_error!("Unexpected end of the json Object (missing column and value)")
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        rust_type!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: rust_type!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::RustType::Null
    };

    (true) => {
        $crate::RustType::Boolean(true)
    };

    (false) => {
        $crate::RustType::Boolean(false)
    };

    ([]) => {
        $crate::RustType::List(Vec::with_capacity(0))
    };

    ([ $($tt:tt)+ ]) => {
        $crate::RustType::List(rust_type!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::RustType::Object(::std::collections::HashMap::with_capacity(0))
    };

    ({ $($tt:tt)+ }) => {
        $crate::RustType::Object({
            let mut object = ::std::collections::HashMap::new();
            rust_type!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Into<RustType> type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {{
        let val: $crate::RustType = $other.into();
        val
    }};
}

#[cfg(test)]
mod tests {
    use crate::rust_type_impl::RustType;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(rust_type!(null)  => RustType::Null)]
    #[test_case(rust_type!(1)     => RustType::Integer(1))]
    #[test_case(rust_type!(2.3)   => RustType::Number(2.3))]
    #[test_case(rust_type!("4")   => RustType::String("4".to_string()))]
    #[test_case(rust_type!(true)  => RustType::Boolean(true))]
    #[test_case(rust_type!(false) => RustType::Boolean(false))]
    #[test_case(rust_type!([])    => RustType::List(Vec::new()))]
    #[test_case(rust_type!({})    => RustType::Object(HashMap::new()))]
    // Test not empty lists
    #[test_case(rust_type!([null]) => RustType::List(vec![RustType::Null]))]
    #[test_case(
        rust_type!([{"k": 6}, [5], {}, [], false, true, "4", 2.3, 1, null]) => RustType::List(vec![
            RustType::Object({
                let mut map = HashMap::new();
                let _ = map.insert("k".to_string(), RustType::Integer(6));
                map
            }),
            RustType::List(vec![RustType::Integer(5)]),
            RustType::Object(HashMap::new()),
            RustType::List(Vec::new()),
            RustType::Boolean(false),
            RustType::Boolean(true),
            RustType::String("4".to_string()),
            RustType::Number(2.3),
            RustType::Integer(1),
            RustType::Null,
        ])
    )]
    // Test not empty maps
    #[test_case(
        rust_type!({
            "{\"k\":6}": {"k": 6},
            "[5]": [5],
            "{}": {},
            "[]": [],
            "false": false,
            "true": true,
            "4": "4",
            "2.3": 2.3,
            "1": 1,
            "null": null
        }) => RustType::Object([
            ("{\"k\":6}".to_string(), RustType::Object({
                let mut map = HashMap::new();
                let _ = map.insert("k".to_string(), RustType::Integer(6));
                map
            })),
            ("[5]".to_string(), RustType::List(vec![RustType::Integer(5)])),
            ("{}".to_string(), RustType::Object(HashMap::new())),
            ("[]".to_string(), RustType::List(Vec::new())),
            ("false".to_string(), RustType::Boolean(false)),
            ("true".to_string(), RustType::Boolean(true)),
            ("4".to_string(), RustType::String("4".to_string())),
            ("2.3".to_string(), RustType::Number(2.3)),
            ("1".to_string(), RustType::Integer(1)),
            ("null".to_string(), RustType::Null),
        ].iter().cloned().collect())
    )]
    #[test_case(
        rust_type!({"null": null}) => RustType::Object([
            ("null".to_string(), RustType::Null)
        ].iter().cloned().collect())
    )]
    const fn test_ensure_macro_is_consistent(value: RustType) -> RustType {
        value
    }
}
