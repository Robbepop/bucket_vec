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

## Benchmarks

`BucketVec` fullfills its role in being a replacement for situations where
a `Vec<Box<T>>` is the naive go-to solution.
The benchmark suite is still small and not super expressive but already provides
some insights in where `BucketVec` already performs pretty well and where it
could improve.

Benchmarks have been run on a Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz.
Note that for every of the benchmark groups (`push`, `get` and `iter`) the
most efficient configuration for the `BucketVec` has been chosen.
Some benchmarks (`get`) have shown significant difference between configs.

The following is the output of a benchmark run:

```
bucket_vec::push/10000       time:   [43.647 us 43.861 us 44.108 us]
vec_box::push/10000          time:   [405.37 us 410.91 us 417.20 us]
vec_value::push/10000        time:   [25.826 us 25.915 us 26.020 us]

bucket_vec::get/10000        time:   [17.369 us 17.416 us 17.470 us]
vec_box::get/10000           time:   [14.446 us 14.485 us 14.537 us]
vec_value::get/10000         time:   [8.7939 us 8.8105 us 8.8300 us]

bucket_vec::iter/10000       time:   [4.4195 us 4.4316 us 4.4454 us]
vec_box::iter/10000          time:   [9.5925 us 9.6246 us 9.6610 us]
vec_value::iter/10000        time:   [3.5955 us 3.6043 us 3.6142 us]

bucket_vec::iter.rev()/10000 time:   [3.9804 us 3.9957 us 4.0144 us]
vec_box::iter.rev()/10000    time:   [9.9677 us 9.9980 us 10.033 us]
vec_value::iter.rev()/10000  time:   [3.5827 us 3.5944 us 3.6080 us]

bucket_vec::iter_mut/10000   time:   [5.0533 us 5.0710 us 5.0909 us]
vec_box::iter_mut/10000      time:   [13.425 us 13.845 us 14.203 us]
vec_value::iter_mut/10000    time:   [4.0172 us 4.0473 us 4.0820 us]
```

It can be seen that `BucketVec` greatly outperforms `Vec<Box<_>>` on
`push`, `iter` and `iter().rev()` benchmarks.
Also `BucketVec` is approximately 50% slower than the `Vec<_>` which is the
theoretical optimum that unfortunately doesn't solve the underlying problem.

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
