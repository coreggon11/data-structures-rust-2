use crate::array::Array;

pub struct ArrayList<T> {
    size: usize,
    array: Array<T>,
}

pub struct ArrayListIterator<T> {
    start: usize,
    end: usize,
    array: Array<T>,
}

impl<T> Iterator for ArrayListIterator<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            let result = self.array.get(self.start);
            self.start += 1;
            result
        }
    }
}

impl<T> DoubleEndedIterator for ArrayListIterator<T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            self.array.get(self.end)
        }
    }
}

impl<T> ArrayList<T>
where
    T: Copy + PartialOrd,
{
    pub fn new() -> ArrayList<T> {
        ArrayList {
            size: 0,
            array: Array::new(4),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        self.array.get(index)
    }

    fn expand(&mut self) {
        let new_array = self.array.copy_with_size(self.size * 2);
        self.array = new_array;
    }

    fn shrink(&mut self) {
        let new_array = self.array.copy_with_size(self.size / 2);
        self.array = new_array;
    }

    pub fn add(&mut self, item: T) {
        if self.array.size() == self.size {
            self.expand();
        }
        self.array.set(self.size, item);
        self.size += 1;
    }

    pub fn set(&mut self, index: usize, item: T) {
        self.array.set(index, item);
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index >= self.size {
            crate::array::array_index_out_of_bounds(index, self.size);
        }
        if self.array.size() == self.size {
            self.expand();
        }
        let new_array: Array<T>;
        let mut left: Array<T>;
        if index != 0 {
            left = self.array.copy_range(0, index);
            let mut help_array: Array<T> = Array::new(1);
            help_array.set(0, item);
            let new_left = left.merge(&help_array, left.size() + 1);
            left = new_left;
        } else {
            left = Array::new(1);
            left.set(0, item);
        }
        let right = self.array.copy_range(index, self.size);
        new_array = left.merge(&right, self.array.size());
        self.array = new_array;
        self.size += 1;
    }

    pub fn remove_item(&mut self, item: T) -> Option<T> {
        let index_to_remove = self.index_of(item);
        if index_to_remove == usize::MAX {
            println!("Item does not exist in this list!");
            None
        } else {
            self.remove(index_to_remove)
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        // invalid index
        if index >= self.size {
            crate::array::array_index_out_of_bounds(index, self.size);
            None
        } else {
            self.size -= 1;
            if self.size == self.array.size() / 4 && self.array.size() > 4 {
                self.shrink(); // if 2 -> 8 --> 4 - the 2nd index we removed will preserve so its safe
            }
            // last index
            if index == self.size - 1 {
                self.array.get(index)
            }
            // other index
            else {
                let item = self.array.get(index);
                let new_array: Array<T>;
                if index == 0 {
                    new_array = self.array.copy(1, self.array.size(), self.array.size());
                } else {
                    let mut left = self.array.copy_range(0, index);
                    let right = self.array.copy_range(index + 1, self.array.size());
                    new_array = left.merge(&right, self.array.size());
                }
                self.array = new_array;
                item
            }
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        if self.array.size() > 4 {
            self.array = Array::new(4);
        }
    }

    fn index_of(&mut self, item: T) -> usize {
        for i in 0..self.size {
            if self.array.get(i).unwrap() == item {
                return i;
            }
        }
        return usize::MAX;
    }

    pub fn into_iter(&mut self) -> ArrayListIterator<T> {
        let size = self.size;
        let array = self.array.clone();

        ArrayListIterator {
            start: 0,
            end: size,
            array: array,
        }
    }
}
