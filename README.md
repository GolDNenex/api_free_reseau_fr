# api_free_reseau_fr

[![Crates.io](https://img.shields.io/crates/v/api_free_reseau_fr.svg)](https://crates.io/crates/api_free_reseau_fr)
[![Docs.rs](https://docs.rs/api_free_reseau_fr/badge.svg)](https://docs.rs/api_free_reseau_fr)
[![CI](https://github.com/goldnenex/api_free_reseau_fr/workflows/CI/badge.svg)](https://github.com/goldnenex/api_free_reseau_fr/actions)

### Rust client for interacting with [free-reseau.fr](https://www.free-reseau.fr/outils/api) API

## Feature

- Async (reqwest)
- serde (feature flag)

## Usage

```rust
use api_free_reseau_fr::{Client, DSLAM, Error};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let target = DSLAM::new("mon", 75, 1);
    //or
    let target = DSLAM::from("mon75-1");

    let status = client.get_dslam_status(&target).await?;

    println!("DSLAM `{target}` is {status}");

    Ok(())
}
```

More examples provided in the [demo](examples/demo.rs)

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
