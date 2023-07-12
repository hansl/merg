// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

#![cfg(feature = "derive")]

use merge::Merge;
use std::collections::HashMap;

/// A macro to create collections (maps or sets) from a list of elements.
/// Can be used as
///
/// ```
/// let vec: Vec<_> = collection![1, 2, 3];
/// let set: HashSet<_> = collection![1, 2, 3];
/// let set: BTreeSet<_> = collection![1, 2, 3];
/// let map: HashMap<_, _> = collection![1 => "a", 2 => "b", 3 => "c"];
/// let map: BTreeMap<_, _> = collection![1 => "a", 2 => "b", 3 => "c"];
/// ```
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

fn test<T: std::fmt::Debug + Merge + PartialEq>(expected: T, mut left: T, right: T) {
    left.merge(right);
    assert_eq!(expected, left);
}

#[test]
fn test_overwrite() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::overwrite)] u8);

    test(S(2), S(1), S(2));

    #[derive(Debug, Merge, PartialEq)]
    struct T(#[merge(strategy = merge::overwrite)] HashMap<u8, &'static str>);

    test(
        T(collection![2 => "b"]),
        T(collection![1 => "a"]),
        T(collection![2 => "b"]),
    );
}

mod option {
    use super::test;
    use crate::Merge;

    #[test]
    fn test_overwrite_none() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::option::overwrite_none)] Option<u8>);

        test(S(Some(1)), S(Some(1)), S(Some(2)));
        test(S(Some(2)), S(None), S(Some(2)));
        test(S(None), S(None), S(None));
    }

    #[test]
    fn test_recursive() {
        #[derive(Debug, Merge, PartialEq)]
        struct N(#[merge(strategy = merge::num::saturating_add)] u8);

        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::option::recurse)] Option<N>);

        test(S(Some(N(3))), S(Some(N(1))), S(Some(N(2))));
        test(S(Some(N(1))), S(Some(N(1))), S(None));
        test(S(Some(N(1))), S(None), S(Some(N(1))));
        test(S(None), S(None), S(None));
    }
}

mod bool {
    use super::test;
    use crate::Merge;

    #[test]
    fn test_overwrite_false() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::bool::overwrite_false)] bool);

        test(S(false), S(false), S(false));
        test(S(true), S(false), S(true));
        test(S(true), S(true), S(false));
        test(S(true), S(true), S(true));
    }

    #[test]
    fn test_overwrite_true() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::bool::overwrite_true)] bool);

        test(S(false), S(false), S(false));
        test(S(false), S(false), S(true));
        test(S(false), S(true), S(false));
        test(S(true), S(true), S(true));
    }
}

mod num {
    use super::test;
    use crate::Merge;

    #[cfg(feature = "num")]
    #[test]
    fn test_saturating_add() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::num::saturating_add)] u8);

        test(S(0), S(0), S(0));
        test(S(1), S(0), S(1));
        test(S(255), S(255), S(10));
        test(S(40), S(30), S(10));
    }

    #[cfg(feature = "num")]
    #[test]
    fn test_overwrite_zero() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::num::overwrite_zero)] u8);

        test(S(0), S(0), S(0));
        test(S(1), S(0), S(1));
        test(S(255), S(255), S(10));
    }

    #[test]
    fn test_ord_max() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::ord::max)] u8);

        test(S(2), S(1), S(2));
        test(S(2), S(2), S(1));
        test(S(2), S(2), S(2));
        test(S(2), S(2), S(0));
        test(S(2), S(0), S(2));
        test(S(33), S(33), S(11));
    }

    #[test]
    fn test_ord_min() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::ord::min)] u8);

        test(S(1), S(1), S(2));
        test(S(1), S(2), S(1));
        test(S(2), S(2), S(2));
        test(S(0), S(2), S(0));
        test(S(0), S(0), S(2));
        test(S(11), S(33), S(11));
    }
}

mod vec {
    use super::test;
    use crate::Merge;

    #[cfg(feature = "std")]
    #[test]
    fn test_vec_overwrite_empty() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::vec::overwrite_empty)] Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![0]), S(vec![0]), S(vec![1]));
        test(S(vec![255]), S(vec![255]), S(vec![10]));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_vec_append() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::vec::append)] Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![0, 1]), S(vec![0]), S(vec![1]));
        test(S(vec![255, 10]), S(vec![255]), S(vec![10]));
        test(S(vec![0, 1, 2, 3, 4]), S(vec![0, 1, 2]), S(vec![3, 4]));
        test(S(vec![3, 4, 0, 1, 2]), S(vec![3, 4]), S(vec![0, 1, 2]));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_vec_prepend() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::vec::prepend)] Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![1, 0]), S(vec![0]), S(vec![1]));
        test(S(vec![10, 255]), S(vec![255]), S(vec![10]));
        test(S(vec![3, 4, 0, 1, 2]), S(vec![0, 1, 2]), S(vec![3, 4]));
        test(S(vec![0, 1, 2, 3, 4]), S(vec![3, 4]), S(vec![0, 1, 2]));
    }
}

#[cfg(feature = "std")]
mod hashmap {
    use super::test;
    use crate::Merge;
    use std::collections::HashMap;

    #[test]
    fn test_overwrite() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::hashmap::overwrite)] HashMap<u8, u8>);

        test(
            S(collection! {1 => 2}),
            S(collection! {1 => 1}),
            S(collection! {1 => 2}),
        );
        test(
            S(collection! {1 => 1}),
            S(collection! {1 => 2}),
            S(collection! {1 => 1}),
        );
        test(
            S(collection! {0 => 1, 1 => 2}),
            S(collection! {0 => 1}),
            S(collection! {1 => 2}),
        );
    }

    #[test]
    fn test_ignore() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::hashmap::ignore)] HashMap<u8, u8>);

        test(
            S(collection! {1 => 1}),
            S(collection! {1 => 1}),
            S(collection! {1 => 2}),
        );
        test(
            S(collection! {1 => 2}),
            S(collection! {1 => 2}),
            S(collection! {1 => 1}),
        );
        test(
            S(collection! {0 => 1, 1 => 2}),
            S(collection! {0 => 1}),
            S(collection! {1 => 2}),
        );
    }

    #[test]
    fn test_recurse() {
        #[derive(Debug, Merge, PartialEq)]
        struct N(#[merge(strategy = merge::num::saturating_add)] u8);

        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::hashmap::recurse)] HashMap<u8, N>);

        test(
            S(collection! {1 => N(3)}),
            S(collection! {1 => N(1)}),
            S(collection! {1 => N(2)}),
        );
        test(
            S(collection! {1 => N(3)}),
            S(collection! {1 => N(2)}),
            S(collection! {1 => N(1)}),
        );
        test(
            S(collection! {0 => N(1), 1 => N(2)}),
            S(collection! {0 => N(1)}),
            S(collection! {1 => N(2)}),
        );
    }
}

#[cfg(feature = "std")]
mod hashset {
    use super::test;
    use crate::Merge;
    use std::collections::HashSet;

    #[test]
    fn test_extend() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = merge::hashset::extend)] HashSet<u8>);

        test(
            S(collection! {1, 2}),
            S(collection! {1}),
            S(collection! {2}),
        );
        test(
            S(collection! {1, 2}),
            S(collection! {1, 2}),
            S(collection! {}),
        );
        test(
            S(collection! {1, 2}),
            S(collection! {}),
            S(collection! {1, 2}),
        );
        test(
            S(collection! {1, 2}),
            S(collection! {1, 2}),
            S(collection! {1, 2}),
        );
        test(
            S(collection! {1, 2, 3}),
            S(collection! {1, 2, 3}),
            S(collection! {1, 2}),
        );
        test(
            S(collection! {1, 2, 3}),
            S(collection! {1, 2}),
            S(collection! {1, 3}),
        );
    }
}
