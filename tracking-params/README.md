# tracking-params

Library to remove various known tracking parameters from a given URL.

Parser rules are defined in [rules.rs](src/rules.rs) file.

```rust
let dirty_url = url::Url::parse("https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5EdUmBgUY").unwrap();
let clean_url = tracking_params::clean(dirty_url); // returns `Cleaned` which derefs to `url::Url`

assert_eq!(
    clean_url.to_string(),
    "https://twitter.com/elonmusk/status/1608273870901096454".to_string() // No `ref_src` tracking params
);
```