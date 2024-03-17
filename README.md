# ✔️ Smoke Tests for Rust Infrastructure

This repository contains a collection of smoke tests for the infrastructure of
the [Rust] project.

## Development

The repository contains a set of tools that enforce a consistent coding style,
run automated tests, and check for common programming errors. Pre-commit hooks
run these checks on the local machine when committing changes, while GitHub
Actions run the same checks when a pull request is opened.

The pre-commit hooks are managed by [pre-commit] and can be installed with the
following command:

```shell
pre-commit install
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[pre-commit]: https://pre-commit.com/
[rust]: https://www.rust-lang.org/
