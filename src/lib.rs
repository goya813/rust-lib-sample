pub mod fibonacci;

#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(21), 10946);
    }
}
