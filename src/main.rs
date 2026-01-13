use std::{alloc::{Layout, alloc, dealloc, realloc}, fmt::{self}, ops::Index, ptr, slice};

fn main() {
    let mut stri = StringImpl::new("Deshan");
    stri.append_char(' ');
    stri.append_str("Anjana");
    
    println!("{}", stri);
    
    let mut buf: RawBuf<u8> = RawBuf::new(4);
    
        buf.push(5).unwrap();
        buf.push(6).unwrap();
        buf.push(12).unwrap();
        buf.push(64).unwrap();
    
    println!("{}", buf);
    
    println!("{}", buf[3]);
    
    let mut x = RawBuf::<f32>::new(5);
    x.push(64.82).unwrap();
    x.push(64.4842).unwrap();
    x.push(1544.82).unwrap();
    
    println!("{}", x);
}

struct RawBuf<T> {
    pointer: *mut T,
    length: usize,
    capacity: usize
}

#[derive(Debug)]
enum RawBufError {
    OutOfSpaceError
}

impl<T> RawBuf<T> {
    fn new(cap: usize) -> Self {
        unsafe {
            let layout = Layout::array::<T>(cap).unwrap();
            let pointer = alloc(layout) as *mut T;
            
            if pointer.is_null() {
                panic!("alloc failed");
            }
            
            Self { pointer: pointer, length: 0, capacity: cap }
        }
    }
    
    fn push(&mut self, byte: T) -> Result<(), RawBufError> {
        if self.length >= self.capacity {
            return Err(RawBufError::OutOfSpaceError);
        }
        unsafe {
            ptr::write(self.pointer.add(self.length), byte);
            self.length += 1;
            Ok(())
        }
    }
}

impl<T: fmt::Debug> fmt::Display for RawBuf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let arr = slice::from_raw_parts(self.pointer, self.length);
            write!(f, "{:?}", arr)
        }
    }
}

impl fmt::Display for RawBufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawBufError::OutOfSpaceError => write!(f, "RawBuf is out of space."),
        }
    }
}

impl<T> Index<usize> for RawBuf<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!("Index {} out of bounds (len = {})", index, self.length);
        }
        unsafe {
            &*self.pointer.add(index)
        }
    }
}

impl<T> Drop for RawBuf<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.length {
                ptr::drop_in_place(self.pointer.add(i));
            }
            
            let layout = Layout::array::<T>(self.capacity).unwrap();
            dealloc(self.pointer as *mut u8, layout);
        }
    }
}

struct StringImpl {
    buffer: *mut u8,
    length: usize,
    capacity: usize
}

impl StringImpl {
    fn new(string: &str) -> Self {
        unsafe {
            let len = string.len();
            let layout = Layout::array::<u8>(len).unwrap();
            let pointer = alloc(layout);
            
            if pointer.is_null() {
                panic!("alloc failed");
            }
            
            ptr::copy_nonoverlapping(string.as_ptr(), pointer, len);
            
            Self { buffer: pointer, length: len, capacity: string.len() }
        }
    }
    
    fn grow_buffer(&mut self, add_len: usize) {
        unsafe {
            if self.length + add_len <= self.capacity {
                return;
            }
            
            let new_cap = (self.capacity.max(add_len) * 2).max(self.length + add_len);
            let new_layout = Layout::array::<u8>(new_cap).unwrap();
            
            let new_ptr = realloc(self.buffer, new_layout, new_cap);
            if new_ptr.is_null() {
                panic!("realloc failed")
            }
            
            self.buffer = new_ptr;
            self.capacity = new_cap;
        }
    }
    
    fn append_char(&mut self, c: char) {
        unsafe {
            self.grow_buffer(1);
            ptr::copy_nonoverlapping(&(c as u8), self.buffer.add(self.length), 1);
            self.length += 1;
        }
    }
    
    fn append_str(&mut self, string: &str) {
        unsafe {
            let add_len = string.len();
            self.grow_buffer(add_len);
            ptr::copy_nonoverlapping(string.as_ptr(), self.buffer.add(self.length), add_len);
            self.length += add_len;
        }
    }
}

impl fmt::Display for StringImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let bytes = slice::from_raw_parts(self.buffer, self.length);
            let s = str::from_utf8_unchecked(bytes);
            f.write_str(s)
        }
    }
}

impl Drop for StringImpl {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<u8>(self.capacity).unwrap();
            dealloc(self.buffer, layout);
        }
    }
}
