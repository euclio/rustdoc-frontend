# rustdoc-static

A frontend that can be used with the new [rustdoc] to generate static
documentation.

## Usage

`rustdoc-static` is intended to be used with the new [rustdoc] as a pluggable
frontend.

To use this frontend instead of the built-in frontend, build the frontend and
set the `RUSTDOC_FRONTEND` environment variable.

```bash
$ cargo build --release
$ export RUSTDOC_FRONTEND=/path/to/repo/target/release/rustdoc-static
```

Now, run `rustdoc` on your target crate. Documentation will be generated in the
crate's `target/doc` directory.

[rustdoc]: https://github.com/steveklabnik/rustdoc

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
