pub(in crate) fn fragment_components_from_fragment(fragment: &str) -> impl Iterator<Item = String> {
    let fragment = fragment.trim_start_matches('/');
    if fragment.is_empty() {
        Vec::with_capacity(0).into_iter()
    } else {
        fragment
            .split('/')
            .map(|fragment_part| fragment_part.replace("~1", "/").replace("~0", "~"))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::fragment_components_from_fragment;
    use test_case::test_case;

    #[test_case("", &[] ; "empty")]
    #[test_case("p1", &["p1"])]
    #[test_case("/p2", &["p2"])]
    #[test_case("p3/p4", &["p3", "p4"])]
    #[test_case("/p5/p6", &["p5", "p6"])]
    #[test_case("/~0/~1", &["~", "/"])]
    fn test_fragment_components_from_fragment(fragment: &str, expected_result: &[&str]) {
        assert_eq!(fragment_components_from_fragment(fragment).collect::<Vec<_>>(), expected_result);
    }
}
