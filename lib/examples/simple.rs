use vaulted::passwords::Password;

fn main() {
    // Create a new password instance
    let mut password = Password::default();
    // Generate a new password of the given length
    password.generate(12);
    // Assert that the password equal to given length
    assert!(password.to_string().len() == 12)
}
