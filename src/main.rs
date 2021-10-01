mod array;
mod array_list;

use array::Array;
use array_list::ArrayList;

fn main() {
    //array_list();
    array();
}

fn array_list() {
    let mut list: ArrayList<&i32> = ArrayList::new();

    list.add(&22);

    let item = list.get(0);
    match item {
        Ok(r) => println!("Got {}", r.unwrap()),
        Err(e) => println!("{}", e),
    }

    println!("List length: {}", list.length());
}

fn array() {
    let mut array: Array<i32> = Array::new(10);

    array.set(0, 15);

    println!("Array length {:?}", array.length());

    let item = array.get(0);
    println!("Got {}", item.unwrap());

    array.clear();
}
