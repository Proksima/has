//! Introduce 'has a' relationship as a trait to Rust.
//!
//! This crate offers an alternative for a missing feature of the Rust Programming Language. That
//! is, the possibility of traits holding state.
//!
//! ## Simple example
//!
//! ```rust
//! #[macro_use]
//! extern crate has;
//!
//! use has::*;
//!
//! struct Apple;
//!
//! trait ApplesContainer: HasMut<Vec<Apple>> {
//!     fn take_apple(&mut self) -> Option<Apple> {
//!         self.get_mut().pop()
//!     }
//!
//!     fn put_apple(&mut self, apple: Apple) {
//!         self.get_mut().push(apple);
//!     }
//! }
//!
//! #[derive(Default)]
//! struct Basket {
//!     pub apples: Vec<Apple>,
//! }
//!
//! impl ApplesContainer for Basket {}
//! impl_has!(Basket, Vec<Apple>, apples);
//!
//! fn main() {
//!     let mut basket = Basket::default();
//!
//!     basket.put_apple(Apple);
//!     basket.put_apple(Apple);
//!     basket.put_apple(Apple);
//!
//!     basket.take_apple();
//!
//!     assert_eq!(basket.apples.len(), 2);
//! }
//! ```
//!

/// Trait to model a "has a" relationship between implementing structs and the generic parameter
/// provided. This trait provides only a function to retrieve a non-mutable reference to the
/// contained object. If a mutable reference is desired instead, use `HasMut`.
///
pub trait Has<T> {
    fn get_ref(&self) -> &T;
}

/// Trait to model a "has a" relationship between implementing structs and the generic parameter
/// provided. This trait provides methods to retrieve either a mutable or immutable reference to
/// the contained object.
///
pub trait HasMut<T>: Has<T> {
    fn get_mut(&mut self) -> &mut T;
}

impl<'a, T, H: Has<T>> Has<T> for &'a H {
    fn get_ref(&self) -> &T {
        Has::<T>::get_ref(*self)
    }
}

impl<'a, T, H: Has<T>> Has<T> for &'a mut H {
    fn get_ref(&self) -> &T {
        Has::<T>::get_ref(*self)
    }
}

impl<'a, T, H: HasMut<T>> HasMut<T> for &'a mut H {
    fn get_mut(&mut self) -> &mut T {
        HasMut::<T>::get_mut(*self)
    }
}

/// Macro to consisely implement `HasMut` for a struct. The macro takes as argument the struct
/// name, the type of the contained object and the identifier, within the struct, of the contained
/// object; in that order.
///
#[macro_export] macro_rules! impl_has {
    ( $implementer:ty, $typ:ty, $identifier:ident ) => {
        impl $crate::Has<$typ> for $implementer {
            fn get_ref(&self) -> &$typ {
                &self.$identifier
            }
        }

        impl $crate::HasMut<$typ> for $implementer {
            fn get_mut(&mut self) -> &mut $typ {
                &mut self.$identifier
            }
        }
    }
}
