# To use this package

1. in your cargo.toml under dependcies add

```toml
candid_gen = { git = "https://github.com/aliscie/autodox-libraries/candid_gen" }
```

2. In your canister folder next to lib.rs create lib.rs file and add this

```rs
#[cfg(test)]
mod tests {
    use candid_gen::generated_candid;
    use super::*;

    #[test]
    fn save_candid() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        generated_candid(current_dir)
    }
}
```

3. Now you can run `carog test`