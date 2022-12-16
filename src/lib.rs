pub const GREETING: &str = "World";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting() {
        let lower_world = GREETING.to_ascii_lowercase();
        assert_eq!(lower_world, "world");
    }
}
