
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
