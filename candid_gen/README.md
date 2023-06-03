# To use this package

1. in your cargo.toml under dependcies add

```toml
candid_gen = { git = "https://github.com/aliscie/autodox-libraries/candid_gen" }
```

2. In your canister folder next to lib.rs create main.rs file and add this

```rs
use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(my_canister_name, modulename, query);
    generate_candid_method!(<you canister name>, modulename, update);
    candid::export_service!();
    std::print!("{}", __export_service());
    std::print!("----------- task is done --------------");
}
```

3. Now you can run `carog run`