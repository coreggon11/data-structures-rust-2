use std::alloc::{self, Layout};
use std::ptr;
use std::ptr::NonNull;

pub struct Array<T> {
    length: usize,
    data: ptr::NonNull<T>,
}

impl<T> Array<T>
where
    T: Copy,
{
    pub fn new(length: usize) -> Self {
        let layout = Layout::array::<T>(length).unwrap();
        let items = unsafe { alloc::alloc(layout) };
        let result_items = match NonNull::new(items as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        Array {
            length: length,
            data: result_items,
        }
    }

    pub fn clear(&mut self) {
        let layout = Layout::array::<T>(self.length).unwrap();
        unsafe {
            alloc::dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            unsafe { Some(ptr::read(self.data.as_ptr().add(index))) }
        } else {
            println!(
                "Invalid array index! Index {}, array length {}",
                index, self.length
            );
            None
        }
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index < self.length {
            unsafe {
                ptr::write(self.data.as_ptr().add(index), item);
            }
        } else {
            println!(
                "Invalid array index! Index {}, array length {}",
                index, self.length
            );
        }
    }
}
