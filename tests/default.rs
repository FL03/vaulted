#[cfg(test)]
mod tests {
    use vaulted::passwords::{validate_password, Password};

    #[test]
    fn test_compiles() {
        let f = |x: usize| x.pow(2);
        let a = f(2);
        let b = 4;
        assert_eq!(a, b)
    }

    #[test]
    fn test_password_builder() {
        let a: String = Password::generate(12).into();
        assert_eq!(a.clone().len(), 12);

        let mut a_prime = Password::new(a.clone());
        a_prime.hash_password();

        let a_hash: String = a_prime.clone().into();
        assert!(validate_password(a.clone(), a_hash));

        let sample_password = "sample".to_string();
        let mut b = Password::new(sample_password.clone());
        b.hash_password();
        assert!(validate_password(sample_password, b.clone().0));

        assert_ne!(a_prime, b);
    }
}
