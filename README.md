# Bucket Vector

|       Docs                       |       Crates.io                        |
|:--------------------------------:|:--------------------------------------:|
| [![docs][docs-badge]][docs-link] | [![crates][crates-badge]][crates-link] |

[docs-badge]: https://docs.rs/bucket_vec/badge.svg
[docs-link]: https://docs.rs/bucket_vec
[crates-badge]: https://img.shields.io/crates/v/bucket_vec.svg
[crates-link]: https://crates.io/crates/bucket_vec

## ⚠️ Caution

> USE WITH CAUTION

As of now this crate has not yet been battle tested
or benchmarked to an extend where the author would recommend general production
usage. Please file bugs, suggestions or enhancements to the issue tracker of [this repository](github.com/Robbepop/bucket-vec).

## Description

A vector-like data structure that organizes its elements into a set of buckets
of fixed-capacity in order to guarantee that mutations to the bucket vector
never moves elements and thus invalidates references to them.

This is comparable to a `Vec<Box<T>>` but a lot more efficient.

## Configs

The `BucketVecConfig` trait allows to customize the internal structure of your
`BucketVec`. This allows users to fine-tune their `BucketVec` for particular
use cases.

The trait mainly controls the capacity of the first bucket and the growth rate
of the capacity of new buckets.

The default `DefaultConfig` tries to balance out the different interests
between start capacity and growth rate.

## Under the Hood

The `BucketVec` is really just a vector of `Bucket` instances.
Whenever an element is pushed to the `BucketVec` the element is pushed onto
the last `Bucket` if it isn't filled, yet.
If the last `Bucket` is filled a new `Bucket` is pushed onto the `BucketVec`
with a new capacity determined by the used bucket vector configuration.

This way the `BucketVec` never moves elements around upon inserting new elements
in order to preserve references. When a normal `Vec` is modified it can potentially
invalidate references because of reallocation of the internal buffer which
might cause severe bugs if references to the internal elements are stored
outside the `Vec`. Note that normally Rust prevents such situations so the
`BucketVec` is mainly used in the area of `unsafe` Rust where a developer
actively decides that they want or need pinned references into another data
structure.

For the same reasons as stated above the `BucketVec` does not allow to remove
or swap elements.

## Example

Looking at an example `BucketVec<i32>` with the following configuration:

- `start_capacity := 1`
- `growth_rate := 2`

We have already pushed the elements `A`,.., `K` onto it.

```
[ [A], [B, C], [D, E, F, G], [H, I, J, K, _, _, _, _] ]
```

Where `_` refers to a vacant bucket entry.

Pushing another `L`,.., `O` onto the same `BucketVec` results in:

```
[ [A], [B, C], [D, E, F, G], [H, I, J, K, L, M, N, O] ]
```

So we are full on capacity for all buckets.
The next time we push another element onto the `BucketVec` it will create a new `Bucket` with a capacity of `16` since `growth_rate == 2` and our current latest bucket already has a capacity of `8`.

```
[ [A], [B, C], [D, E, F, G], [H, I, J, K, L, M, N, O], [P, 15 x _] ]
```

Where `15 x _` denotes 15 consecutive vacant entries.

## Authors & Credits

Author: Robin Freyler (github.com/Robbepop)

Special thanks to Niklas Tittjung (github.com/lugino-emeritus) who helped me a
lot with some internal formulae.

## License

Licensed under either of

 * Apache license, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Dual licence: [![badge][license-mit-badge]](LICENSE-MIT) [![badge][license-apache-badge]](LICENSE-APACHE)

[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-apache-badge]: https://img.shields.io/badge/license-APACHE-orange.svg
