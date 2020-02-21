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
     Running target/release/deps/bench-88691f185efb078f
bucket_vec::push/10000  time:   [45.263 us 45.568 us 45.949 us]
                        change: [-1.0498% +1.5709% +4.5268%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 23 outliers among 100 measurements (23.00%)
  17 (17.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high severe

vec_box::push/10000     time:   [418.10 us 424.25 us 431.12 us]
                        change: [-85.039% -1.9304% +548.90%] (p = 0.85 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

vec_value::push/10000   time:   [26.439 us 26.570 us 26.721 us]
                        change: [-3.5568% +0.2897% +4.9941%] (p = 0.90 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  10 (10.00%) low severe
  3 (3.00%) high mild
  1 (1.00%) high severe

bucket_vec::get/10000   time:   [17.907 us 17.999 us 18.109 us]
                        change: [-1.3982% -0.8478% -0.2357%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

vec_box::get/10000      time:   [15.033 us 15.089 us 15.146 us]
                        change: [-0.6401% -0.0157% +0.6349%] (p = 0.97 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

vec_value::get/10000    time:   [9.1707 us 9.2141 us 9.2654 us]
                        change: [+0.0592% +0.7703% +1.5666%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

bucket_vec::iter/10000  time:   [31.203 us 31.299 us 31.401 us]
                        change: [-0.3489% +0.1717% +0.6862%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

vec_box::iter/10000     time:   [9.8021 us 9.8342 us 9.8689 us]
                        change: [-1.9532% -1.4184% -0.8723%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

vec_value::iter/10000   time:   [3.7098 us 3.7219 us 3.7349 us]
                        change: [+0.3577% +1.3855% +2.8360%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
```

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
