use crate::array_list::ArrayList;

#[derive(Copy, Clone, PartialEq)]
pub struct HeapItem<T> {
    priority: i64,
    data: T,
}

pub struct Heap<T> {
    list: ArrayList<HeapItem<T>>,
}

impl<T> Heap<T>
where
    T: Copy + PartialEq,
{
    pub fn new() -> Heap<T> {
        Heap {
            list: ArrayList::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }

    pub fn push(&mut self, priority: i64, data: T) {
        let item = HeapItem {
            priority: priority,
            data: data,
        };
        self.list.add(item);
        let mut current_index = self.list.size() - 1;
        let mut parent_index = self.parent_index(current_index);
        loop {
            let current_item = self.list.get(current_index).unwrap(); // we know its not null
            let parent = self.list.get(parent_index).unwrap(); //same lol
            if current_item.priority >= parent.priority {
                break;
            }
            self.list.swap(current_index, parent_index);
            current_index = parent_index;
            parent_index = self.parent_index(current_index);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.list.size() == 0 {
            println!("Empty heap!");
            None
        } else {
            let out = self.list.get(0);
            if self.list.size() > 1 {
                let last = self.list.get(self.list.size() - 1);
                self.list.set(0, last.unwrap());
                let mut current_index = 0;
                let mut son_index = self.greater_son_index(current_index);
                loop {
                    let current_item = self.list.get(current_index).unwrap();
                    let son = self.list.get(son_index).unwrap();
                    if son_index >= self.list.size() || current_item.priority <= son.priority {
                        break;
                    }
                    self.list.swap(current_index, son_index);
                    current_index = son_index;
                    son_index = self.greater_son_index(current_index);
                }
            }
            Some(out.unwrap().data)
        }
    }

    pub fn peek(&mut self) -> Option<T> {
        if self.list.size() == 0 {
            println!("Empty heap!");
            None
        } else {
            Some(self.list.get(0).unwrap().data)
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn peek_priority(&mut self) -> i64 {
        if self.list.size() == 0 {
            println!("Empty heap!");
            0
        } else {
            self.list.get(0).unwrap().priority
        }
    }

    fn parent_index(&mut self, index: usize) -> usize {
        if index <= 1 {
            0
        } else {
            (index - 1) / 2
        }
    }

    fn greater_son_index(&mut self, index: usize) -> usize {
        let right_son = (2 * index) + 2;
        let left_son = (2 * index) + 1;
        if left_son >= self.list.size() || right_son >= self.list.size() {
            return left_son;
        } else {
            return if self.list.get(left_son).unwrap().priority
                <= self.list.get(right_son).unwrap().priority
            {
                left_son
            } else {
                right_son
            };
        }
    }
}
