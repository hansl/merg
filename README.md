# merg

The `merg` crate provides the `Merge` trait that can be used to merge multiple
values into one:

```rust
trait Merge {
    fn merge(&mut self, other: Self);
}
```

`Merge` can be derived for structs:

<!-- should be kept in sync with examples/user.rs -->

```rust
use merg::Merge;

#[derive(Merge)]
struct User {
    // Fields with the skip attribute are skipped by Merge
    #[merge(skip)]
    pub name: &'static str,

    // The strategy attribute is used to select the merge behavior
    #[merge(strategy = merg::option::overwrite_none)]
    pub location: Option<&'static str>,

    #[merge(strategy = merg::vec::append)]
    pub groups: Vec<&'static str>,
}

let defaults = User {
name: "",
location: Some("Internet"),
groups: vec!["rust"],
};
let mut ferris = User {
name: "Ferris",
location: None,
groups: vec!["mascot"],
};
ferris.merge(defaults);

assert_eq!("Ferris", ferris.name);
assert_eq!(Some("Internet"), ferris.location);
assert_eq!(vec!["mascot", "rust"], ferris.groups);
```

A merge strategy is a function with the signature `fn merge<T>(left: &mut T,
right: T)` that merges `right` into `left`. The `merge` crate provides
strategies for the most common types, but you can also define your own
strategies.

The trait can be used to merge configuration from different sources, for
example environment variables, multiple configuration files and command-line
arguments, see the `args.rs` example.

## Features

This crate has the following features:

- `derive` (default):  Enables the derive macro for the `Merge` trait using the
  `merg_derive` crate.
- `num` (default): Enables the merge strategies in the `num` module that
  require the `num_traits` crate.
- `std` (default): Enables the merge strategies in the `hashmap` and `vec`
  modules that require the standard library. If this feature is not set,
  `merg` is a `no_std` library.

## Minimum Supported Rust Version

This crate supports Rust 1.36.0 or later.

## Contact

This fork can be found at https://github.com/hansl/merg.

## License

This project is dual-licensed under the [Apache-2.0][] and [MIT][] licenses.

[Apache-2.0]: https://opensource.org/licenses/Apache-2.0

[MIT]: https://opensource.org/licenses/MIT
