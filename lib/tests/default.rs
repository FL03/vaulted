#[cfg(test)]
mod tests {
    #[test]
    fn test_compiles() {
        let f = |x: usize| x.pow(2);
        let a = f(2);
        let b = 4;
        assert_eq!(a, b)
    }
}
