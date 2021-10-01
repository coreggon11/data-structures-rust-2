use crate::array::Array;

pub struct ArrayList<T> {
    size: usize,
    array: Array<T>,
}

impl<T> ArrayList<T>
where
    T: Copy,
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
        let new_array = self.array.copy_expanded(self.size * 2);
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
            left = self.array.copy(0, index);
            let mut help_array: Array<T> = Array::new(1);
            help_array.set(0, item);
            let new_left = left.merge(&help_array, left.size() + 1);
            left = new_left;
        } else {
            left = Array::new(1);
            left.set(0, item);
        }
        let mut right = self.array.copy(index, self.size);
        new_array = left.merge(&right, self.array.size());
        self.array = new_array;
        self.array.set(self.size, item);
        self.size += 1;
    }

    pub fn remove_item(&mut self, item: T) {
        // TODO
    }

    pub fn remove(&mut self, index: usize) {
        // TODO
    }

    pub fn clear(&mut self) {
        // TODO
    }

    fn index_of(&mut self, item: T) {
        // TODO
    }
}
