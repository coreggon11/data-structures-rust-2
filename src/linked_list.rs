use std::ptr;

pub fn list_index_out_of_bounds(index: usize, length: usize) {
    println!(
        "Linked list index out of bounds! Index {}, array length {}",
        index, length
    );
}

pub struct LinkedListItem<T> {
    data: T,
    next: *const LinkedListItem<T>,
    previous: *const LinkedListItem<T>,
}

pub struct LinkedList<T> {
    size: usize,
    begin: *const LinkedListItem<T>,
    end: *const LinkedListItem<T>,
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
            begin: ptr::null(),
            end: ptr::null(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn get_node(&mut self, index: usize) -> LinkedListItem<T> {
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
        unsafe { ptr::read(item) }
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
            None
        } else {
            Some(self.get_node(index).data)
        }
    }

    pub fn add(&mut self, item: T) {
        if self.size == 0 {
            let new_item = LinkedListItem {
                data: item,
                next: ptr::null(),
                previous: ptr::null(),
            };
            self.begin = &new_item;
            self.end = &new_item;
        } else {
            let new_item = LinkedListItem {
                data: item,
                next: ptr::null(),
                previous: self.end,
            };
            self.end = &new_item;
        }
        self.size += 1;
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
        } else {
            self.get_node(index).data = item;
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
        } else {
            self.size += 1;
            let mut new_node = LinkedListItem {
                data: item,
                next: ptr::null(),
                previous: ptr::null(),
            };
            if index == 0 {
                new_node.next = self.begin;
                self.begin = &new_node;
            } else {
                let mut node = self.get_node(index);
                new_node.previous = &node;
                new_node.next = node.next;
                node.next = &new_node;
                unsafe {
                    ptr::read(new_node.next).previous = &new_node;
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
        unsafe { self.remove_node(&ptr::read(current_node)) }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            list_index_out_of_bounds(index, self.size);
            None
        } else {
            let node = self.get_node(index);
            self.remove_node(&node)
        }
    }

    fn remove_node(&mut self, node: *const LinkedListItem<T>) -> Option<T> {
        let out: T;
        if self.size == 1 {
            unsafe {
                out = ptr::read(self.begin).data;
            }
            self.begin = ptr::null();
            self.end = ptr::null();
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
        self.size -= 1;
        Some(out)
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.begin = ptr::null();
        self.end = ptr::null();
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
