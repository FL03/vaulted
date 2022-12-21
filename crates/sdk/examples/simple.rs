use vaulted_sdk::prelude::{Password, PasswordBuilder};

fn main() {
    // Initialize a new builder
    let mut builder = PasswordBuilder::default();
    // Generate a new password of the given length
    builder.generate(12);
    let password: Password = builder.password().clone();

    // Assert that the password equal to given length
    assert!(password.to_string().len() == 12)
}
