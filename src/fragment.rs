pub(in crate) fn fragment_components_from_fragment(fragment: &str) -> Box<Iterator<Item = String>> {
    let fragment = fragment.trim_start_matches('/');
    if fragment.is_empty() {
        Box::new(Vec::with_capacity(0).into_iter())
    } else {
        Box::new(
            fragment
                .split('/')
                .map(|fragment_part| fragment_part.replace("~1", "/").replace("~0", "~"))
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}
