pub fn hello() -> String {
    ("Hello").to_string()
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello");
    }
}