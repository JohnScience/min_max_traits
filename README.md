# Traits for MIN and MAX associated constants

At the time of writing, all [primitive numeric types] in Rust provide `MIN` and `MAX` [associated constants](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants), which nonetheless do not belong to any [trait].

One commonly used crate, [`num-traits`], offers many useful traits for numeric types. However, the closest 
analogue of `min_max_traits::Min` and `min_max_traits::Max` offered by [`num-traits`] at the time of writing
is [`num_traits::Bounded`], which requires implementation of `min_value()` and `max_value()` functions. Since [`const_fn_trait_bound`](https://doc.rust-lang.org/nightly/unstable-book/language-features/const-fn-trait-bound.html) feature is in the works, [`num_traits::Bounded`] cannot be used in generic implementations of constant functions
relying on `MIN` and `MAX` [associated constants], at least on stable Rust.

These [traits][trait] can be useful, for example, to generically implement [associated constants] storing the
greatest length of primitive integers when converted to strings.

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1


## Floating-point types

The [IEEE 754-2008](https://en.wikipedia.org/wiki/IEEE_754) "binary32" and "binary64" floating-point types are `f32` and
`f64`, respectively.

[`num-traits`]: https://crates.io/crates/num-traits
[primitive numeric types]: https://doc.rust-lang.org/stable/reference/types/numeric.html
[trait]: https://doc.rust-lang.org/book/ch10-02-traits.html
[`num_traits::Bounded`]: https://docs.rs/num-traits/0.2.14/num_traits/bounds/trait.Bounded.html
[associated constants]: https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants