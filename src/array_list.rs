pub struct ArrayList<T> {
    size: usize,
    length: usize,
    items: [Option<T>; 4],
}

impl<T> ArrayList<T>
where
    T: Copy,
{
    pub fn new() -> ArrayList<T> {
        ArrayList {
            size: 4,
            length: 0,
            items: [None; 4],
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn get(&mut self, index: usize) -> Result<Option<T>, &'static str> {
        if self.length > index {
            Result::Ok(self.items[index])
        } else {
            Result::Err("Invalid array list index!")
        }
    }

    pub fn add(&mut self, item: T) {
        let arr_length = self.length;
        if arr_length == self.size {
            // TODO expand
        } else {
            self.items[arr_length] = Some(item);
            self.length += 1;
        }
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
