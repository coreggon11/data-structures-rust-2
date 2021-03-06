mod array;
mod array_list;
mod heap;
mod linked_list;

use array::Array;
use array_list::ArrayList;
use heap::Heap;
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
            "heap" => heap(),
            _ => println!("Invalid program argument"),
        }
    }
}

fn heap() {
    let mut heap: Heap<i32> = Heap::new();

    // lower number -> higher priority
    heap.push(46, 1);
    heap.push(42, 2);
    heap.push(45, 3);
    heap.push(27, 4);

    assert_eq!(heap.size(), 4);

    heap.pop();
    assert_eq!(heap.peek().unwrap(), 2);
    assert_eq!(heap.peek_priority(), 42);

    heap.clear();
    assert_eq!(heap.size(), 0);
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
    list.add(2);

    assert_eq!(3, list.size());

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

    list.set(0, 11);
    {
        let item = list.get(0);
        assert_eq!(
            11,
            item.unwrap(),
            "Item expected {} was {}",
            11,
            item.unwrap()
        );
    }

    list.set(1, 12);
    {
        let item = list.get(1);
        assert_eq!(
            12,
            item.unwrap(),
            "Item expected {} was {}",
            12,
            item.unwrap()
        );
    }

    list.insert(0, 55);
    {
        let item = list.get(0);
        assert_eq!(
            55,
            item.unwrap(),
            "Item expected {} was {}",
            55,
            item.unwrap()
        );
        assert_eq!(4, list.size());
    }

    list.insert(1, 22);
    {
        let item = list.get(1);
        assert_eq!(
            22,
            item.unwrap(),
            "Item expected {} was {}",
            22,
            item.unwrap()
        );
        assert_eq!(5, list.size());
    }

    {
        let item = list.remove_item(55);
        assert_eq!(
            55,
            item.unwrap(),
            "Item expected {} was {}",
            55,
            item.unwrap()
        );
        assert_eq!(4, list.size());
    }

    {
        let item = list.remove_item(11);
        assert_eq!(
            11,
            item.unwrap(),
            "Item expected {} was {}",
            11,
            item.unwrap()
        );
        assert_eq!(3, list.size());
    }

    {
        let item = list.remove_item(2);
        assert_eq!(
            2,
            item.unwrap(),
            "Item expected {} was {}",
            2,
            item.unwrap()
        );
        assert_eq!(2, list.size());
    }

    list.add(3);
    list.add(4);
    list.add(5);

    {
        let item = list.remove(0);
        assert_eq!(
            22,
            item.unwrap(),
            "Item expected {} was {}",
            22,
            item.unwrap()
        );
        assert_eq!(4, list.size());
    }

    {
        let item = list.remove(1);
        assert_eq!(
            3,
            item.unwrap(),
            "Item expected {} was {}",
            3,
            item.unwrap()
        );
        assert_eq!(3, list.size());
    }
    {
        let item = list.remove(2);
        assert_eq!(
            5,
            item.unwrap(),
            "Item expected {} was {}",
            5,
            item.unwrap()
        );
        assert_eq!(2, list.size());
    }

    let mut index = 0;
    for i in list.into_iter() {
        println!("Index {}, data {}", index, i);
        index += 1;
    }

    list.clear();
}
