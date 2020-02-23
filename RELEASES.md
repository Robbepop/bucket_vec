# Releases

Release history for `bucket_vec` crate.

## 0.7 - 2020-02-23

- Implement `IntoIterator` for `&BucketVec` and `&mut BucketVec`
- Implement `parity-scale-codec::{Encode, Decode}` for `BucketVec`
    - Opt-in crate feature: `scale-1`

## 0.6 - 2020-02-23

- Losen trait bound constraints for
    - `Clone`
    - `PartialEq`
    - `Eq`
    - `PartialOrd`
    - `Ord`
    - `Hash`

    trait implementations of `BucketVec`.
- Implement `IntoIterator` for `BucketVec`

## 0.5 - 2020-02-23

- Add derive for `Clone` for `BucketVec`
- Add `BucketVec::iter_mut`
- Improve performance of `BucketVec::iter`
- Restructured crate slightly

## 0.4 - 2020-02-22

- Slightly improve performance of `BucketVec::{push, get}` for natural `GROWTH_RATE` config
- Greatly improve performance of `BucketVec::iter`
- Fix bug with `no_std` compat

## 0.3 - 2020-02-22

- Optimize `BucketVec::get` for `GROWTH_RATE` of `2.0`
- Lift trait bound constraints for `C` generic parameter for `BucketVec::iter`

## 0.2 - 2020-02-21

- derive `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` for `BucketVec`
- implement `core::iter::Extend` for `BucketVec`
- export `Iter` publicly
- implement `FromIterator<Item = T>` for `BucketVec<T>`

## 0.1.1

- fix bug with `BucketVec::{get, get_mut}` index calculations
- implement `no_std` compat

## 0.1

- initial release
