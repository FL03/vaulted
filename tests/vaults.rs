

#[cfg(test)]
mod tests {
    use vaulted::vaults::VaultAccess;

    #[test]
    fn test_vault_access_default() {
        let a = VaultAccess::default();
        let b = VaultAccess::None;
        assert_eq!(a, b)
    }
}

