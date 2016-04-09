## 'Has a' relationship for Rust's traits

This crate offers an alternative for a missing feature of the Rust Programming Language. That
is, the possibility for traits to hold states.

[[![](http://meritbadge.herokuapp.com/has)](https://crates.io/crates/has)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Travis Build Status](https://travis-ci.org/Proksima/has.svg?branch=master)](https://travis-ci.org/Proksima/has)

### Documentation

http://proksima.github.io/has-doc/has/index.html

### Simple example

```rust
#[macro_use]
extern crate has;

use has::*;

struct Apple;

trait ApplesContainer: HasMut<Vec<Apple>> {
    fn take_apple(&mut self) -> Option<Apple> {
        self.get_mut().pop()
    }

    fn put_apple(&mut self, apple: Apple) {
        self.get_mut().push(apple);
    }
}

#[derive(Default)]
struct Basket {
    pub apples: Vec<Apple>,
}

impl ApplesContainer for Basket {}
impl_has!(Basket, Vec<Apple>, apples);

fn main() {
    let mut basket = Basket::default();

    basket.put_apple(Apple);
    basket.put_apple(Apple);
    basket.put_apple(Apple);

    basket.take_apple();

    assert_eq!(basket.apples.len(), 2);
}
```

