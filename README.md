rust-example-use
==

[![Actions Status](https://github.com/mitchallen/rust-example-use/workflows/Build%20Test%20Run/badge.svg)](https://github.com/mitchallen/rust-example-use/actions)


An example of using a GitHub hosted Rust dependency

## Usage

```sh
make all
```

## Cargo.toml

```toml
[dependencies]
rust-example-lib = { git = "https://github.com/mitchallen/rust-example-lib.git", tag = "v0.1.1" }
```

## Code usage

**NOTE** that `rust-example-lib` is converted to `rust_example_lib`:

```rs
let _result = rust_example_lib::coin::flip();
```


## Dependency Repo

* https://github.com/mitchallen/rust-example-lib
