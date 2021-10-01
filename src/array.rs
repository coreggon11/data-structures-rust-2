use std::alloc::{self, Layout};
use std::ptr;
use std::ptr::NonNull;

pub fn array_index_out_of_bounds(index: usize, length: usize) {
    println!(
        "Array index out of bounds! Index {}, array length {}",
        index, length
    );
}

pub struct Array<T> {
    size: usize,
    data: ptr::NonNull<T>,
}

pub struct ArrayIterator<T> {
    start: *const T,
    end: *const T,
}

impl<T> Iterator for ArrayIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
}

impl<T> DoubleEndedIterator for ArrayIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Array<T>
where
    T: Copy,
{
    pub fn new(length: usize) -> Self {
        let layout = Layout::array::<T>(length).unwrap();
        let data = unsafe { alloc::alloc_zeroed(layout) };
        let result_data = match NonNull::new(data as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        Array {
            size: length,
            data: result_data,
        }
    }

    pub fn copy_expanded(&mut self, length: usize) -> Array<T> {
        let layout = Layout::array::<T>(length).unwrap();
        let data = unsafe { alloc::alloc(layout) };
        let mut result_data = match NonNull::new(data as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        unsafe {
            ptr::copy_nonoverlapping(self.data.as_mut(), result_data.as_mut(), self.size);
        }
        Array {
            size: length,
            data: result_data,
        }
    }

    pub fn copy(&mut self, from: usize, to: usize) -> Array<T> {
        if to < from {
            panic!("Index 'to' must be less than index 'from'");
        }
        if from > self.size {
            panic!("Index 'from' must be less than array size");
        }
        if to > self.size {
            panic!("Index 'to' must be less than array size");
        }
        let layout = Layout::array::<T>(to - from).unwrap();
        let data = unsafe { alloc::alloc(layout) };
        let mut result_data = match NonNull::new(data as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        unsafe {
            ptr::copy_nonoverlapping(
                self.data.as_ptr().add(from),
                result_data.as_mut(),
                to - from,
            );
        }
        Array {
            size: to - from,
            data: result_data,
        }
    }

    pub fn merge(&mut self, other: &Array<T>, size: usize) -> Array<T> {
        let layout = Layout::array::<T>(size).unwrap();
        let data = unsafe { alloc::alloc(layout) };
        let mut result_data = match NonNull::new(data as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        unsafe {
            ptr::copy_nonoverlapping(self.data.as_ptr(), result_data.as_mut(), self.size);
            ptr::copy_nonoverlapping(
                other.data.as_ptr(),
                result_data.as_ptr().add(self.size),
                other.size,
            )
        }
        Array {
            size: size,
            data: result_data,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        if index < self.size {
            unsafe { Some(ptr::read(self.data.as_ptr().add(index))) }
        } else {
            array_index_out_of_bounds(index, self.size());
            None
        }
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index < self.size {
            unsafe {
                ptr::write(self.data.as_ptr().add(index), item);
            }
        } else {
            array_index_out_of_bounds(index, self.size());
        }
    }

    pub fn into_iter(self) -> ArrayIterator<T> {
        let data = self.data;
        let size = self.size;

        std::mem::forget(self);

        unsafe {
            ArrayIterator {
                start: data.as_ptr(),
                end: if size == 0 {
                    data.as_ptr()
                } else {
                    data.as_ptr().add(size)
                },
            }
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.size).unwrap();
        unsafe {
            alloc::dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }
}
