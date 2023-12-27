extern crate hashmap_rust;
use hashmap_rust::HashMap;

fn main() {
    let _timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();
}
