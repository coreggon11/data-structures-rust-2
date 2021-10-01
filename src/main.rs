mod array;
mod array_list;

use array::Array;
use array_list::ArrayList;

fn main() {
    array();
    //array_list();
}

fn array_list() {
    let mut list: ArrayList<i32> = ArrayList::new();

    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);

    let item = list.get(3);
    println!("Got {}", item.unwrap());

    list.insert(0, 11);

    let item = list.get(0);
    println!("Got {}", item.unwrap());

    println!("List length: {}", list.size());
}

fn array() {
    let mut array: Array<i32> = Array::new(10);

    array.set(0, 15);

    println!("Array length {:?}", array.size());

    let item = array.get(0);
    println!("Got {}", item.unwrap());

    let mut index = 0;
    for i in array.into_iter() {
        println!("Index {}, data {}", index, i);
        index += 1;
    }
}
