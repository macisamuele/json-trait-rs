use crate::json_type::JsonType;

// Prevent users from implementing the Index trait. (Idea extrapolated from libcore/slice/mod.rs)
mod private {
    pub trait Sealed {}

    impl Sealed for usize {}

    impl Sealed for &str {}

    impl Sealed for String {}
}

pub trait Index<T>: private::Sealed
where
    T: JsonType,
{
    fn index_into<'json>(&self, v: &'json T) -> Option<&'json T>;
}

impl<T> Index<T> for usize
where
    T: JsonType,
{
    #[inline]
    fn index_into<'json>(&self, v: &'json T) -> Option<&'json T> {
        v.get_index(*self)
    }
}

impl<T> Index<T> for &str
where
    T: JsonType,
{
    #[inline]
    fn index_into<'json>(&self, v: &'json T) -> Option<&'json T> {
        v.get_attribute(self)
    }
}

impl<T> Index<T> for String
where
    T: JsonType,
{
    #[inline]
    fn index_into<'json>(&self, v: &'json T) -> Option<&'json T> {
        v.get_attribute(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Index;
    use crate::{json_type::JsonType, rust_type::RustType};

    #[test]
    fn test_into_index_vec() {
        let testing_vec: RustType = rust_type_vec![(), true, "v3", 4, rust_type_vec![1, 2, 3], rust_type_map![],];

        let testing_object = testing_vec.clone();
        for (k, v) in testing_vec.as_array().unwrap().enumerate() {
            assert_eq!(k.index_into(&testing_object), Some(v), "failed with k={}\n", k);
        }

        assert_eq!(
            (testing_vec.as_array().unwrap().len() + 1).index_into(&testing_object),
            None,
            "failed to extract not existing index (k={}\n)",
            testing_vec.as_array().unwrap().len() + 1,
        );
    }

    #[test]
    fn test_into_index_map_str() {
        let testing_map = rust_type_map![
            "k1" => (),
            "k2" => true,
            "k3" => "v3",
            "k4" => 4,
            "k5" => rust_type_vec![1,2,3],
            "k6" => rust_type_map![],
        ];

        if let RustType::Object(ref hash_map) = &testing_map {
            for (k, v) in hash_map {
                assert_eq!(k.as_str().index_into(&testing_map), Some(v), "failed with k={}\n", k);
            }

            assert_eq!(
                "not-existing-key".index_into(&testing_map),
                None,
                "failed to extract not existing index (k={}\n)",
                "not-existing-key",
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_into_index_map_string() {
        let testing_map = rust_type_map![
            "k1" => (),
            "k2" => true,
            "k3" => "v3",
            "k4" => 4,
            "k5" => rust_type_vec![1,2,3],
            "k6" => rust_type_map![],
        ];

        if let RustType::Object(ref hash_map) = &testing_map {
            for (k, v) in hash_map {
                assert_eq!(k.index_into(&testing_map), Some(v), "failed with k={}\n", k);
            }

            assert_eq!(
                String::from("not-existing-key").index_into(&testing_map),
                None,
                "failed to extract not existing index (k={}\n)",
                "not-existing-key",
            );
        } else {
            unreachable!();
        }
    }
}
