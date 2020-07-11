pub fn greet (name: &str) -> String {
    return format!("Hello, {}!", name);
}


#[cfg(test)]
mod tests {
    use super::greet;
    #[test]
    fn test_greet() {
        assert_eq!("Hello, Papaya!", greet("Papaya"));
    }
}
