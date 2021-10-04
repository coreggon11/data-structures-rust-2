use std::boxed;
use std::ptr;

pub fn list_index_out_of_bounds(index: usize, length: usize) {
    println!(
        "Linked list index out of bounds! Index {}, array length {}",
        index, length
    );
}

pub struct LinkedListItem<T> {
    data: T,
    next: *mut LinkedListItem<T>,
    previous: *mut LinkedListItem<T>,
}

pub struct LinkedList<T> {
    size: usize,
    begin: *mut LinkedListItem<T>,
    end: *mut LinkedListItem<T>,
}

pub struct LinkedListIterator<T> {
    begin: *const LinkedListItem<T>,
    end: *const LinkedListItem<T>,
}

impl<T> Iterator for LinkedListIterator<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.begin == ptr::null() {
            None
        } else {
            unsafe {
                let item = ptr::read(self.begin);
                let result = item.data;
                self.begin = item.next;
                Some(result)
            }
        }
    }
}

impl<T> DoubleEndedIterator for LinkedListIterator<T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<T> {
        if self.end == ptr::null() {
            None
        } else {
            unsafe {
                let item = ptr::read(self.end);
                let result = item.data;
                self.begin = item.previous;
                Some(result)
            }
        }
    }
}

impl<T> LinkedList<T>
where
    T: Copy + PartialOrd,
{
    pub fn new() -> LinkedList<T> {
        LinkedList {
            size: 0,
            begin: ptr::null_mut(),
            end: ptr::null_mut(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn get_node(&mut self, index: usize) -> *mut LinkedListItem<T> {
        let begin = if index <= self.size / 2 { true } else { false };
        let mut item = if begin { self.begin } else { self.end };
        let mut current_index = if begin { 0 } else { self.size - 1 };
        loop {
            if current_index == index {
                break;
            }
            unsafe {
                item = if begin {
                    ptr::read(item).next
                } else {
                    ptr::read(item).previous
                }
            }
            if begin {
                current_index += 1;
            } else {
                current_index -= 1;
            }
        }
        item
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
            None
        } else {
            unsafe { Some(ptr::read(self.get_node(index)).data) }
        }
    }

    pub fn add(&mut self, item: T) {
        if self.size == 0 {
            let new_item = LinkedListItem {
                data: item,
                next: ptr::null_mut(),
                previous: ptr::null_mut(),
            };
            self.begin = Box::into_raw(Box::new(new_item));
            self.end = self.begin;
        } else {
            let new_item = LinkedListItem {
                data: item,
                next: ptr::null_mut(),
                previous: self.end,
            };
            unsafe {
                let mut end = ptr::read(self.end);
                let new_ptr = Box::into_raw(Box::new(new_item));
                end.next = new_ptr;
                ptr::drop_in_place(self.end);
                ptr::write(self.end, end);
                self.end = new_ptr;
            }
        }
        self.size += 1;
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
        } else {
            unsafe {
                let ptr = self.get_node(index);
                let mut copy = ptr::read(ptr);
                copy.data = item;
                ptr::drop_in_place(ptr);
                ptr::write(ptr, copy);
            }
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
        } else {
            self.size += 1;
            let mut new_node = LinkedListItem {
                data: item,
                next: ptr::null_mut(),
                previous: ptr::null_mut(),
            };
            if index == 0 {
                new_node.next = self.begin;
                let new_ptr = Box::into_raw(Box::new(new_node));
                unsafe {
                    let mut begin = ptr::read(self.begin);
                    begin.previous = new_ptr;
                    ptr::drop_in_place(self.begin);
                    ptr::write(self.begin, begin);
                    self.begin = new_ptr;
                }
            } else {
                let node = self.get_node(index);
                new_node.previous = node;
                unsafe {
                    new_node.next = ptr::read(node).next;
                    let next_ptr = new_node.next;
                    let new_ptr = Box::into_raw(Box::new(new_node));
                    let mut node_value = ptr::read(node);
                    node_value.next = new_ptr;
                    ptr::drop_in_place(node);
                    ptr::write(node, node_value);
                    let mut node_value_next = ptr::read(next_ptr);
                    node_value_next.previous = new_ptr;
                    ptr::drop_in_place(next_ptr);
                    ptr::write(next_ptr, node_value_next);
                }
            }
        }
    }

    pub fn remove_item(&mut self, item: T) -> Option<T> {
        let mut current_node = self.begin;
        let mut current_index = 0;
        loop {
            if current_index >= self.size {
                return None;
            }
            unsafe {
                if ptr::read(current_node).data == item {
                    break;
                }
                current_node = ptr::read(current_node).next;
            }
            current_index += 1;
        }
        self.remove_node(current_node)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
            None
        } else {
            let node = self.get_node(index);
            self.remove_node(node)
        }
    }

    fn remove_node(&mut self, node: *mut LinkedListItem<T>) -> Option<T> {
        let out: T;
        if self.size == 1 {
            unsafe {
                out = ptr::read(self.begin).data;
            }
            self.begin = ptr::null_mut();
            self.end = ptr::null_mut();
        } else if node == self.begin {
            unsafe {
                out = ptr::read(self.begin).data;
                self.begin = ptr::read(self.begin).next;
            }
        } else if node == self.end {
            unsafe {
                out = ptr::read(self.end).data;
                self.end = ptr::read(self.end).previous;
            }
        } else {
            unsafe {
                out = ptr::read(node).data;
                let next = ptr::read(node).next;
                let previous = ptr::read(node).previous;
                ptr::read(previous).next = next;
                ptr::read(next).previous = previous;
            }
        }
        unsafe {
            ptr::drop_in_place(node);
        }
        self.size -= 1;
        Some(out)
    }

    pub fn clear(&mut self) {
        self.size = 0;
        let mut current = self.begin;
        loop {
            if current == ptr::null_mut() {
                break;
            }
            unsafe {
                let next = ptr::read(current).next;
                ptr::drop_in_place(current);
                current = next;
            }
        }
        self.begin = ptr::null_mut();
        self.end = ptr::null_mut();
    }

    pub fn into_iter(&self) -> LinkedListIterator<T> {
        let begin = self.begin;
        let end = self.end;

        LinkedListIterator {
            begin: begin,
            end: end,
        }
    }
}
