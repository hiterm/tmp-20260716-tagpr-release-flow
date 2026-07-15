pub fn released_version_is_specific(reference: &str) -> bool {
    !reference.ends_with(":latest")
}

pub fn references_version(reference: &str, version: &str) -> bool {
    reference.ends_with(&format!(":{version}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_latest() {
        assert!(!released_version_is_specific("example.invalid/app:latest"));
        assert!(released_version_is_specific("example.invalid/app:2.8.1"));
    }

    #[test]
    fn accepts_the_expected_version() {
        assert!(references_version("example.invalid/app:2.8.1", "2.8.1"));
        assert!(!references_version("example.invalid/app:2.8.0", "2.8.1"));
    }
}
