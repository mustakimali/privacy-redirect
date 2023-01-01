<h1 align="center">tracking-params</h1>
<div align="center">
 <strong>
   Library to remove various known tracking parameters from a given URL.
 </strong>
</div>

<br />

<div align="center">
  <a href="https://crates.io/crates/tracking-params">
    <img src="https://img.shields.io/crates/v/tracking-params.svg?style=flat-square" alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/tracking-params">
    <img src="https://img.shields.io/crates/d/tracking-params.svg?style=flat-square" alt="Download" />
  </a>
  <a href="https://docs.rs/tracking-params">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" />
  </a>
</div>
<br/>

Parser rules are defined in [rules.rs](src/rules.rs) file.

## Example
```rust
let dirty_url = url::Url::parse("https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5EdUmBgUY").unwrap();
let clean_url = tracking_params::clean(dirty_url); // returns `Cleaned` which derefs to `url::Url`

assert_eq!(
    clean_url.to_string(),
    "https://twitter.com/elonmusk/status/1608273870901096454".to_string() // No `ref_src` tracking params
);
```