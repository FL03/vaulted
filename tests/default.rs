#[cfg(test)]

mod tests {
    use argon2::{Argon2, PasswordVerifier};
    use vaulted::passwords::{Password, validate_password};

    #[test]
    fn test_compiles() {
        let f = |x: usize| x.pow(2);
        let a = f(2);
        let b = 4;
        assert_eq!(a, b)
    }

    #[test]
    fn test_password_builder() {
        let a = Password::generate(12).0;
        let mut a_prime = Password::new(a.clone());
        let a_hash = a_prime.hash_password().clone().0;

        let sample_password = "sample".to_string();
        let mut b = Password::new(sample_password.clone());
        b.hash_password();
        assert!(validate_password(a.clone(), a_hash));
        assert!(validate_password(sample_password, b.clone().0));
        assert_eq!(a.len(), 12);
    }

}
