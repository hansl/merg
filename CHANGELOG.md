<!---
SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# Unreleased

- Remove the `Merge` implementation for `Option<T>`.
- Add new merge strategies:
  - `overwrite`
  - `option::overwrite_none`
  - `option::overwrite_some`
  - `option::recurse`
  - `hashmap::overwrite`
  - `hashmap::ignore`
  - `hashmap::recurse`
  - `hashset::extend`

# v0.1.0 (2020-09-01)

Initial release providing the `Merge` trait and some merge strategies in the
`bool`, `num`, `ord` and `vec` modules.
