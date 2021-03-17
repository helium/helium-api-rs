# helium-api

An async library to access the public [Helium](https://helium.com) blockchain REST API.

[![Crates.io][crates-badge]][crates-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/helium-api.svg
[crates-url]: https://crates.io/crates/helium-api
[actions-badge]: https://github.com/helium/helium-api-rs/workflows/Continuous%20Integration/badge.svg
[actions-url]: https://github.com/helium/helium-api-rs/actions?query=workflow%3AContinuous%20Integration+branch%3Amaster

## Overview

The Helium API is a REST API service as defined by the
[blockhain-http](https://github.com/helium/blockchain-http) service. This
library attempts to wrap this API in an async, easy to use library that supports
the conventions as exposed by the API. This includes:

* Modular access to each of the main areas of the Helium API
* Support for lazily fetched paged responses

Contributions and helpful suggestions are [always
welcome](https://github.com/helium/helium-api-rs/issues)

## Example

Create a client to the default `api.helium.io` endpoint and ask for a given
account.

```rust,no-run
use helium_api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let account = accounts::get(
        &client,
        "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH",
    )
    .await?;
    println!("Account: {:?}", account);
    Ok(())
}
```

See the examples folder and unit tests for more examples.
