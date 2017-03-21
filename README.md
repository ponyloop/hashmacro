# Hash macro
Rust macro for fast HashMap initialisation.


## Usage

```rust
#[macro_use]
extern crate hashmacro;
use std::collections::HashMap;

fn main () {
  let hashmap: HashMap<_, u32> = hashmap! {
   foo: 2u32,
   bar: 33u32
 };
}
