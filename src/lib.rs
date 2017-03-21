//! Rust macro for fast HashMap initialisation.
//!
//! # Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate hashmacro;
//! use std::collections::HashMap;
//!
//! fn main () {
//!	  let hashmap: HashMap<_, u32> = hashmap! {
//!     foo: 2u32,
//!     bar: 33u32
//!   };
//! }
//! ```


#[macro_export]
macro_rules! hashmap {

	( $($key:ident : $value:expr),* ) => {{
		let mut hash = HashMap::<&str, _>::new();
		$(
			hash.insert(stringify!($key), $value);
		)*
		
		hash
	}};
}
