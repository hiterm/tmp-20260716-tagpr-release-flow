pub fn released_version_is_specific(reference: &str) -> bool {
    !reference.ends_with(":latest")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_latest() {
        assert!(!released_version_is_specific("example.invalid/app:latest"));
        assert!(released_version_is_specific("example.invalid/app:2.8.1"));
    }
}
