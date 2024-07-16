pub fn str_contains(source: &str, search: &[&str]) -> bool {
    for s in search {
        if source.contains(s) {
            return true;
        }
    }
    false
}
