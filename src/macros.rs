#[macro_export]
macro_rules! testing_map {
    ($($k:expr => $v: expr),*,) => {{
        testing_map![$($k => $v),*]
    }};
    ($($k: expr => $v: expr),*) => {{
        use crate::testing::TestingType;
        use std::collections::hash_map::HashMap;

        // Variable definition is needed to ensure that the resulting type is known in the context
        #[allow(unused_mut)]
        let mut thing: HashMap<String, TestingType> = HashMap::default();
        $( let _ = thing.insert($k.to_string(), TestingType::from($v)); )*
        TestingType::from(thing)
    }};
}

#[macro_export]
macro_rules! testing_vec {
    ($($item: expr),*,) => {{
        testing_vec![$($item),*]
    }};
    ($($item: expr),*) => {{
        use crate::testing::TestingType;

        // Variable definition is needed to ensure that the resulting type is known in the context
        let thing: Vec<TestingType> = vec![
            $( TestingType::from($item), )*
        ];
        TestingType::from(thing)
    }};
}
