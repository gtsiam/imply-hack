# imply-hack: Implied bounds, since 1.79!

Add implied bounds to your traits by adding `Imply` as a super trait:

```rust
trait Bound {}

trait MyTrait<T>: Imply<T, Is: Bound> {} // Implies T: Bound
```

Works with Rust 1.79+.

For more information, see the [documentation](https://docs.rs/imply-hack).
