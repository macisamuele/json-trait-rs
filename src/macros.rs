#[macro_export]
macro_rules! rust_type_map {
    ($($k:expr => $v: expr),*,) => {{
        rust_type_map![$($k => $v),*]
    }};
    ($($k: expr => $v: expr),*) => {{
        use $crate::RustType;
        use std::collections::hash_map::HashMap;

        // Variable definition is needed to ensure that the resulting type is known in the context
        #[allow(unused_mut)]
        let mut thing: HashMap<String, RustType> = HashMap::default();
        $( let _ = thing.insert($k.to_string(), RustType::from($v)); )*
        RustType::from(thing)
    }};
}

#[macro_export]
macro_rules! rust_type_vec {
    ($($item: expr),*,) => {{
        rust_type_vec![$($item),*]
    }};
    ($($item: expr),*) => {{
        use $crate::RustType;

        // Variable definition is needed to ensure that the resulting type is known in the context
        let thing: Vec<RustType> = vec![
            $( RustType::from($item), )*
        ];
        RustType::from(thing)
    }};
}
