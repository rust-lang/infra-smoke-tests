# ✔️ Smoke Tests for Rust Infrastructure

This repository contains a collection of smoke tests for the infrastructure of
the [Rust] project. The tests mostly check that the Content Delivery Networks
that the project uses for its releases and crates return the expected responses.

```shell
$ just run
✅ crates.io
  ✅ rust-lang/crates.io#4891 - Encoded + character
    ✅ CloudFront encoded
    ✅ CloudFront unencoded
    ✅ CloudFront with space
    ✅ Fastly encoded
    ✅ Fastly unencoded
    ✅ Fastly with space
  ✅ rust-lang/crates.io#6164 - CORS headers
    ✅ CloudFront
    ✅ Fastly
  ✅ Database dumps
    ✅ CloudFront
    ✅ Fastly

✅ rustup
  ✅ rustup.sh
    ✅ CloudFront
    ✅ Fastly
  ✅ win.rustup.rs
    ✅ aarch64
    ✅ i686
    ✅ x86_64
```

## Usage

This project uses [Just](https://github.com/casey/just) as a command runner. It
is recommended to install and use it with this project, but the commands that
`just` runs can also be copied from the [Justfile](./Justfile) at the root of
the repository.

The smoke tests can be run with the following command:

```shell
just run
```

The smoke tests are organized in the following way:

- _Test suites_ execute tests for a specific service, for example `crates.io`
  or`rustup`.
- Each test suite has one or more _test groups_, which are collections of tests
  that check a specific aspect of the service. Usually, they correspond to a
  feature or a bug that was fixed in the past.
- Each test group has one or more _tests_, which usually execute a single HTTP
  request and check the response. A common pattern is to have a test for each
  CDN, for example.

If a test fails, the test failure and its error message will be bubbled up the
chain and mark the whole test suite as failed.

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

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[pre-commit]: https://pre-commit.com/
[rust]: https://www.rust-lang.org/
