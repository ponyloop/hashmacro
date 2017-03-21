
#[macro_use]
extern crate hashmacro;
use std::collections::HashMap;


#[test]
fn it_returns_a_hashmap() {
	let map: HashMap<_, u32> = hashmap! {
		key: 2u32,
		key2: 33u32
	};

	let mut tmp_map: HashMap<_, u32> = HashMap::new();
	tmp_map.insert("key", 2u32);
	tmp_map.insert("key2", 33u32);

	let cmp_map = tmp_map;

	assert_eq!(map, cmp_map)
}
