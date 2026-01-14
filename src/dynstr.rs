use std::{alloc::{Layout, alloc, dealloc, realloc}, fmt, ptr::{self, NonNull}, slice};

pub(crate) struct DynamicString {
    buff: NonNull<u8>,
    len: usize,
    cap: usize
}

impl DynamicString {
    pub fn new(s: &str) -> Self {
        unsafe {
            if s.len() == 0 { return Self { buff: NonNull::dangling(), len: 0, cap: 0 }; }
            
            let s_len = s.len();
            
            let layout = Layout::array::<u8>(s_len).unwrap();
            let p = alloc(layout);
            
            if p.is_null() { panic!("alloc failed."); }
            
            ptr::copy_nonoverlapping(s.as_ptr(), p, s_len);
            
            Self { buff: NonNull::new(p).expect("alloc failed"), len: s_len, cap: s_len }
        }
    }
    
    fn grow_buffer(&mut self, additinal: usize) {
        unsafe {
            if additinal + self.len <= self.cap { return; }
            
            let new_cap = (self.cap.max(additinal) * 2).max(additinal + self.len);
            let old_layout = Layout::array::<u8>(self.cap).unwrap();
            
            let p = realloc(self.buff.as_ptr(), old_layout, new_cap * std::mem::size_of::<u8>());
            if p.is_null() { panic!("realloc failed"); }
            
            self.buff = NonNull::new(p).expect("alloc failed");
            self.cap = new_cap;
        }
    }
    
    pub fn append_char(&mut self, c: char) {
        unsafe {
            self.grow_buffer(1);
            ptr::copy_nonoverlapping(&(c as u8), self.buff.as_ptr().add(self.len), 1);
            self.len += 1;
        }
    }
    
    pub fn append_str(&mut self, s: &str) {
        unsafe {
            let add_len = s.len();
            self.grow_buffer(add_len);
            ptr::copy_nonoverlapping(s.as_ptr(), self.buff.as_ptr().add(self.len), add_len);
            self.len += add_len;
        }
    }
}

impl fmt::Display for DynamicString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let bytes = slice::from_raw_parts(self.buff.as_ptr(), self.len);
            let s = str::from_utf8_unchecked(bytes);
            f.write_str(s)
        }
    }
}

impl Drop for DynamicString {
    fn drop(&mut self) {
        unsafe {
            if self.cap == 0 { return; }
            
            let layout = Layout::array::<u8>(self.cap).unwrap();
            dealloc(self.buff.as_ptr(), layout);
        }
    }
}