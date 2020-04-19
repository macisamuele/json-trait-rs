pub fn fragment_components_from_fragment(fragment: &str) -> impl Iterator<Item = String> {
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

pub fn fragment_from_fragment_components<I: IntoIterator<Item = T>, T: ToString>(fragment_components: I) -> String {
    fragment_components
        .into_iter()
        .map(|fragment_part| fragment_part.to_string().replace("~", "~0").replace("/", "~1"))
        .fold("".to_string(), |mut result, item| {
            result.push('/');
            result.push_str(&item);
            result
        })
}

#[cfg(test)]
mod tests {
    use super::{fragment_components_from_fragment, fragment_from_fragment_components};
    use test_case::test_case;

    #[test_case(""       => Vec::<String>::new() ; "empty")]
    #[test_case("p1"     => vec!["p1"]           ; "one level withoout leading slash")]
    #[test_case("/p2"    => vec!["p2"]           ; "one level with leading slash")]
    #[test_case("p3/p4"  => vec!["p3", "p4"]     ; "two levels without leading slash")]
    #[test_case("/p5/p6" => vec!["p5", "p6"]     ; "two levels with leading slash")]
    #[test_case("/~0/~1" => vec!["~", "/"]       ; "two levels with special characters")]
    fn test_fragment_components_from_fragment(fragment: &str) -> Vec<String> {
        fragment_components_from_fragment(fragment).collect::<Vec<_>>()
    }

    #[test_case(vec![]           => ""       ; "empty")]
    #[test_case(vec!["p1"]       => "/p1"    ; "one level")]
    #[test_case(vec!["p2", "p3"] => "/p2/p3" ; "two levels")]
    #[test_case(vec!["~", "/"]   => "/~0/~1" ; "two levels with special characters")]
    fn test_fragment_from_fragment_components(fragment_components: Vec<&str>) -> String {
        fragment_from_fragment_components(fragment_components)
    }
}
