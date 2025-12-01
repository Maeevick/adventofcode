pub fn hello() -> &'static str {
    "Hello, advent of code with rust!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_ok() {
        let result = hello();
        assert_eq!(result, "Hello, advent of code with rust!");
    }
}
