//! Add implied bounds to your traits by adding [`Imply`] as a super trait:
//!
//! ```rust
//! trait Bound {}
//!
//! trait MyTrait<T>: Imply<T, Is: Bound> {} // Implies T: Bound
//! ```
//!
//! Works with Rust 1.79+.
//!
//! For more information, see [Why](#the-problem) and [How](#how-it-works).
//!
//! ## The problem
//!
//! If you're the type of person to get lost deep into generic code, you might have run into something
//! like this:
//!
//! ```rust
//! trait MyTrait<T> {
//!     fn do_the_thing(value: &T);
//! }
//!
//! struct Foo;
//!
//! struct MyFooUser;
//!
//! impl MyFooUser {
//!     fn use_value<T>(&self, value: &T)
//!     where
//!         Foo: MyTrait<T>
//!     {
//!         Foo.do_the_thing(value)
//!     }
//! }
//!
//! fn run<T>(value: &T, user: MyFooUser)
//! where
//!     Foo: MyTrait<T>,
//! {
//!     MyFooUser.use_value(&value);
//!
//!     Foo.do_the_thing(&value); // Do it again!
//! }
//! ```
//!
//! Now, this is all well and good. But suppose we now want to make `run` generic over any `FooUser`.
//!
//! ```rust
//! trait FooUser<T>
//! {
//!     fn use_value(&self, value: &T);
//! }
//!
//! impl<T> FooUser<T> for MyFooUser
//! where
//!     Foo: MyTrait<T>
//! {
//!     fn use_value(&self, value: &T) { /* ... */ }
//! }
//! ```
//!
//! Now, suppose that `FooUser<T>` only really makes sense when `Foo: MyTrait<T>` and, notice that we
//! use `Foo: MyTrait<T>` both in `run` and in the implementation of `FooUser<T>`.
//!
//! We're violating one of the most important rules of software development: Dont Repeat Yourself!
//!
//! > Note: It might not seem that big of a deal in this example, but imagine that what I'm representing
//! > here as a simple bound is, in fact, decidedly **not** simple. And that `run` is not just a
//! > function, but a trait implementation. One of many.
//!
//! ```rust
//! fn run<T, U>(value: T, user: U)
//! where
//!     U: FooUser<T>,
//!     Foo: MyTrait<T>, // We really want to get rid of this.
//! {
//!     user.use_value(&value)
//!     Foo.do_the_thing(&value);
//! }
//! ```
//!
//! If you've run into similar situations before, you might be tempted to do:
//!
//! ```rust
//! trait FooUser<T>
//! where
//!     Foo: MyTrait<T>
//! { /* ... */ }
//!
//! fn run<T, U>(value: T, user: U)
//! where
//!     U: FooUser<T>,
//! { /* ... */ }
//! ```
//! But this does not quite work...
//!
//! ```text
//! error[E0277]: the trait bound `Foo: MyTrait<T>` is not satisfied
//!   --> src/lib.rs:31:8
//!    |
//! 31 |     U: FooUser<T>,
//!    |        ^^^^^^^^^^ the trait `MyTrait<T>` is not implemented for `Foo`
//!    |
//! note: required by a bound in `FooUser`
//!   --> src/lib.rs:26:10
//!    |
//! 24 | trait FooUser<T>
//!    |       ------- required by a bound in this trait
//! 25 | where
//! 26 |     Foo: MyTrait<T>
//!    |          ^^^^^^^^^^ required by this bound in `FooUser`
//! ```
//!
//! Congratulations! You just stumbled into [RFC 2089: Extended Implied
//! bounds](https://rust-lang.github.io/rfcs/2089-implied-bounds.html)
//!
//! ## Possible solutions
//!
//! 1. Wait for stabilization (See you in 10 years).
//! 2. Grab a copy of rustc, zulip and get coding!
//! 3. Bite the bullet, and start typing bounds.
//! 4. Rework the entire architecture.
//!
//! None of these is particularly appealing if you don't want to start a very big side quest.
//!
//! Or...
//!
//! 5. Use this crate.
//!
//! ## How it works
//!
//! Suppose we want `MyTrait` to imply `T: Bound`.
//!
//! Rust 1.79 [stabilized](https://github.com/rust-lang/rust/pull/122055/#issue-2170532454) implied
//! bounds on super traits and, notably, associated bounds of super traits.
//!
//! We can use this by creating a supertrait for `MyTrait`, and then constraining an associated type on
//! that super trait (which we set equal to `T`), such that it satisfies `Bound`. This looks like this:
//!
//! ```rust
//! trait Imply {
//!     type Is;
//! }
//!
//! trait MyTrait<T>
//! where
//!     Self: Imply<Is: Bound>,
//!     Self: Imply<Is = T>,
//! {}
//! ```
//!
//! This is still a bit annoying to use. Refining the design a bit we get:
//!
//! ```rust
//! trait Imply<T>: ImplyInner<T, Is = T> {}
//!
//! trait ImplyInner<T> {
//!     type Is;
//! }
//!
//! trait MyTrait<T>: Imply<T, Is: Bound> {}
//! ```
//!
//! Then, add a few blanket impls and we have `imply_hack`!

/// Creates an implied bound when applied as a supertrait.
///
/// ```rust
/// trait MyTrait<T>: Imply<T, Is: Bound> {} // Implies T: Bound
/// ```
pub trait Imply<T>: sealed::ImplyInner<T, Is = T> {}

impl<T, U> Imply<T> for U {}

mod sealed {
    pub trait ImplyInner<T> {
        type Is;
    }

    impl<T, U> ImplyInner<T> for U {
        type Is = T;
    }
}
