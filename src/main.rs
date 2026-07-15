fn release_message(version: &str) -> String {
    format!("tagpr release flow fixture {version}")
}

fn main() {
    println!("{}", release_message(env!("CARGO_PKG_VERSION")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_is_healthy() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn message_contains_the_version() {
        assert_eq!(release_message("2.8.1"), "tagpr release flow fixture 2.8.1");
    }
}
