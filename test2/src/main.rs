use itertools::Itertools;

use std::collections::HashMap;

fn main() {
    let tuples = [("one", 1), ("two", 2), ("three", 3)];
    let m = tuples.into_iter().collect::<HashMap<_, _>>();
    println!("{:?}", m);

    let m2 = tuples.into_iter().map(|x| (x.0, x.1 * 2)).collect::<HashMap<_, _>>();
    println!("{:?}", m2);

    let m3 = [("a", 1), ("b", 2), ("c", 3), ("a", 2), ("b", 3)]
        .iter()
        .into_group_map_by(|i| i.0)
        .into_iter()
        .collect::<HashMap<&str, Vec<&(&str, i32)>>>();
    println!("{:?}", m3);

}
