#![cfg(feature = "serde")]

use btree_slab::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
    map: BTreeMap<i32, i32>,
}

#[test]
fn test_serde() {
    let map: BTreeMap<i32, i32> = BTreeMap::new();
}
