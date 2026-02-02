use std::{alloc::{Layout, alloc, dealloc, realloc}, fmt, mem::ManuallyDrop, ops::Index, ptr::{self, NonNull}, slice};

pub(crate) struct DynamicArray<T> {
    buff: NonNull<T>,
    len: usize,
    cap: usize
}

impl<T> DynamicArray<T> {
    pub fn new<const N: usize>(array: [T; N]) -> Self {
        unsafe {
            let layout = Layout::array::<T>(N).unwrap();
            let p = alloc(layout) as *mut T;
            
            if p.is_null() { panic!("alloc failed"); }
            
            let arr = ManuallyDrop::new(array);
            
            for i in 0..N {
                ptr::write(p.add(i), ptr::read(&arr[i]));
            }
            
            Self { buff: NonNull::new(p).unwrap(), len: N, cap: N }
        }
    }
    
    pub fn reserve(capacity: usize) -> Self {
        unsafe {
            let layout = Layout::array::<T>(capacity).unwrap();
            let p = alloc(layout) as *mut T;
            
            if p.is_null() { panic!("alloc failed"); }
            
            Self { buff: NonNull::new(p).unwrap(), len: 0, cap: capacity }
        }
    }
    
    fn grow_buffer(&mut self) {
        unsafe {
            let new_cap = self.cap + self.cap / 2 + 5;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            
            let p = realloc(self.buff.as_ptr() as *mut u8, old_layout, new_cap * std::mem::size_of::<T>()) as *mut T;
            if p.is_null() { panic!("realloc failed"); }
            
            self.buff = NonNull::new(p).unwrap();
            self.cap = new_cap;
        }
    }
    
    pub fn push(&mut self, bytes: T) {
        unsafe {
            if self.len == self.cap { self.grow_buffer(); }
            ptr::write(self.buff.as_ptr().add(self.len), bytes);
            self.len += 1;
        }
    }
    
    pub fn pop(&mut self) {
        if self.len == 0 { panic!("Nothing to pop") }
        unsafe {
            ptr::drop_in_place(self.buff.as_ptr().add(self.len));
            self.len -= 1;
        }
    }
}

impl<T: fmt::Debug> fmt::Display for DynamicArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let s = slice::from_raw_parts(self.buff.as_ptr(), self.len);
            write!(f, "{:?}", s)
        }
    }
}

impl<T> Index<usize> for DynamicArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.cap { panic!("Out of bounds") }
        unsafe { &*self.buff.as_ptr().add(index) }
    }
}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.buff.as_ptr().add(i));
            }
            
            let layout = Layout::array::<T>(self.cap).unwrap();
            dealloc(self.buff.as_ptr() as *mut u8, layout);
        }
    }
}