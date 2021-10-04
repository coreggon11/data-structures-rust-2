mod array;
mod array_list;
mod linked_list;

use array::Array;
use array_list::ArrayList;
use linked_list::LinkedList;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("No program argument given!");
    } else {
        match args[1].as_ref() {
            "array" => array(),
            "array_list" => array_list(),
            "linked_list" => linked_list(),
            _ => println!("Invalid program argument"),
        }
    }
}

fn array_list() {
    let mut list: ArrayList<i32> = ArrayList::new();

    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);

    assert_eq!(
        list.size(),
        5,
        "Array list size expected {} was {}",
        5,
        list.size(),
    );

    {
        let item = list.get(3);
        assert_eq!(
            item.unwrap(),
            4,
            "3rd index expected {} was {}",
            4,
            item.unwrap(),
        );
    }

    list.insert(0, 11);

    {
        let item = list.get(0);
        assert_eq!(
            item.unwrap(),
            11,
            "0th index expected {} was {}",
            11,
            item.unwrap(),
        );
    }

    assert_eq!(
        list.size(),
        6,
        "List size expected {} was {}",
        6,
        list.size(),
    );

    {
        let item = list.get(5);
        assert_eq!(
            item.unwrap(),
            5,
            "5th index expected {} was {}",
            5,
            item.unwrap()
        );
    }

    list.set(3, 10);
    {
        let item = list.get(3);
        assert_eq!(
            item.unwrap(),
            10,
            "3rd index expected {} was {}",
            10,
            item.unwrap()
        );
    }

    {
        let item = list.remove(3);
        assert_eq!(
            item.unwrap(),
            10,
            "Removed index expected {} was {}",
            10,
            item.unwrap()
        );
    }

    assert_eq!(
        list.size(),
        5,
        "List size expected {} was {}",
        5,
        list.size(),
    );

    {
        let item = list.remove(0);
        assert_eq!(
            item.unwrap(),
            11,
            "Removed index expected {} was {}",
            11,
            item.unwrap()
        );
    }

    assert_eq!(
        list.size(),
        4,
        "List size expected {} was {}",
        4,
        list.size(),
    );

    {
        let item = list.remove_item(2);
        assert_eq!(
            item.unwrap(),
            2,
            "Removed index expected {} was {}",
            2,
            item.unwrap()
        );
    }

    assert_eq!(
        list.size(),
        3,
        "List size expected {} was {}",
        3,
        list.size(),
    );

    let mut index = 0;
    for i in list.into_iter() {
        println!("Index {}, data {}", index, i);
        index += 1;
    }

    list.clear();

    assert_eq!(
        list.size(),
        0,
        "List size expected {} was {}",
        0,
        list.size(),
    );
}

fn array() {
    let mut array: Array<i32> = Array::new(10);

    array.set(0, 15);

    assert_eq!(
        array.size(),
        10,
        "Array size should be {} was {}",
        10,
        array.size()
    );

    let item = array.get(0);
    assert_eq!(
        item.unwrap(),
        15,
        "0th index should be {} was {}",
        15,
        item.unwrap()
    );

    let mut index = 0;
    for i in array.into_iter() {
        println!("Index {}, data {}", index, i);
        index += 1;
    }
}

fn linked_list() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.add(0);
    list.add(1);

    assert_eq!(2, list.size());

    {
        let item = list.get(0);
        assert_eq!(0, item.unwrap());
    }
    {
        let item = list.get(1);
        assert_eq!(
            1,
            item.unwrap(),
            "Item expected {} was {}",
            1,
            item.unwrap()
        );
    }
    //pub fn add(&mut self, item: T) {
    //pub fn set(&mut self, index: usize, item: T) {
    //pub fn insert(&mut self, index: usize, item: T) {
    //pub fn remove_item(&mut self, item: T) -> Option<T> {
    //pub fn remove(&mut self, index: usize) -> Option<T> {
    // pub fn clear(&mut self) {
    //pub fn into_iter(&self) -> LinkedListIterator<T> {
    // list.clear();
}
