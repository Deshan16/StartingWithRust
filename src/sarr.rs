use std::{alloc::{Layout, alloc, dealloc}, fmt, ops::Index, ptr::{self, NonNull}, slice};

pub(crate) struct StaticArray<T> {
    buff: NonNull<T>,
    len: usize,
    cap: usize
}

#[derive(Debug)]
pub enum StaticArrayError {
    OutOfSpaceError
}

impl<T> StaticArray<T> {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let layout = Layout::array::<T>(capacity).unwrap();
            let p = alloc(layout) as *mut T;
            
            if p.is_null() { panic!("alloc failed"); }
            
            Self { buff: NonNull::new(p).expect("alloc failed"), len: 0, cap: capacity }
        }
    }
    
    pub fn push(&mut self, data: T) -> Result<(), StaticArrayError> {
        if self.len >= self.cap { return Err(StaticArrayError::OutOfSpaceError);}
        unsafe {
            ptr::write(self.buff.as_ptr().add(self.len), data);
            self.len += 1;
            Ok(())
        }
    }
}

impl<T: fmt::Debug> fmt::Display for StaticArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(f, "{:?}", slice::from_raw_parts(self.buff.as_ptr(), self.len))
        }
    }
}

impl<T> Index<usize> for StaticArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.cap { panic!("Out of bounds.") }
        unsafe { &*self.buff.as_ptr().add(index) }
    }
}

impl<T> Drop for StaticArray<T> {
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

impl fmt::Display for StaticArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StaticArrayError::OutOfSpaceError => write!(f, "Buffer is out of space."),
        }
    }
}