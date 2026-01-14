use std::{alloc::{Layout, alloc, dealloc, realloc}, fmt, ptr, slice};

pub(crate) struct DynamicArray<T> {
    buff: *mut T,
    len: usize,
    cap: usize
}

impl<T> DynamicArray<T> {
    pub fn reserve(capacity: usize) -> Self {
        unsafe {
            let layout = Layout::array::<T>(capacity).unwrap();
            let p = alloc(layout) as *mut T;
            
            if p.is_null() { panic!("alloc failed"); }
            
            Self { buff: p, len: 0, cap: capacity }
        }
    }
    
    fn grow_buffer(&mut self, additinal: usize) {
        unsafe {
            if additinal + self.len <= self.cap { return; }
            
            let new_cap = (self.cap.max(additinal) * 2).max(additinal + self.len);
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            
            let p = realloc(self.buff as *mut u8, old_layout, new_cap * std::mem::size_of::<T>()) as *mut T;
            if p.is_null() { panic!("realloc failed"); }
            
            self.buff = p;
            self.cap = new_cap;
        }
    }
    
    pub fn push(&mut self, bytes: T) {
        unsafe {
            self.grow_buffer(1);
            ptr::write(self.buff.add(self.len), bytes);
            self.len += 1;
        }
    }
}

impl<T: fmt::Debug> fmt::Display for DynamicArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let s = slice::from_raw_parts(self.buff, self.len);
            write!(f, "{:?}", s)
        }
    }
}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.buff.add(i));
            }
            
            let layout = Layout::array::<T>(self.cap).unwrap();
            dealloc(self.buff as *mut u8, layout);
        }
    }
}