use std::{alloc::{Layout, alloc, dealloc, realloc}, arch::x86_64::{__m128i, _mm_cmpeq_epi8, _mm_loadl_epi64, _mm_movemask_epi8, _mm_set1_epi8}, fmt, ops::AddAssign, ptr::{self, NonNull}, slice};

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
    
    pub fn length(&self) -> usize {
        return self.len;
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
    
    pub fn as_str(&mut self) -> &str {
        unsafe {
            let s = slice::from_raw_parts(self.buff.as_ptr(), self.len);
            std::str::from_utf8_unchecked(s)
        }
    }
    
    pub fn pop(&mut self) {
        if self.len == 0 { panic!("Nothing to pop") }
        unsafe {
            ptr::drop_in_place(self.buff.as_ptr().add(self.len));
            self.len -= 1;
        }
    }
    
    pub fn index_of(&self, c: char) -> Option<usize> {
        unsafe {
            let l = self.len;
            let mut i: usize = 0;
            
            if l / 8 > 1 {
                let look_mask = _mm_set1_epi8(c as i8);
                
                while i < l - (l % 8) {
                    let chunk = _mm_loadl_epi64(self.buff.as_ptr().add(i) as *const __m128i);
                    let mask = _mm_movemask_epi8(_mm_cmpeq_epi8(chunk, look_mask)) & 0xFF;
                    
                    if mask != 0 { return Some(i + mask.trailing_zeros() as usize) }
                    
                    i += 8;
                }
            }
            
            while i < l - 1 {
                if *self.buff.as_ptr().add(i) as char == c { return Some(i); }
            }
            
            None
        }
    }
    
    pub fn find_pattern(&self, pattern: &str) -> bool {
        let index = match self.index_of((*pattern.as_bytes().get(0).unwrap()) as char) {
            Some(i) => i,
            None => return false,
        };
        
        if pattern.len() + index >= self.len { return false; }
        
        unsafe {
            for i in 1..pattern.len() {
                if *pattern.as_bytes().get(i).unwrap() != *self.buff.as_ptr().add(index + i) { return false; }
            }
        }
        
        return true;
    }
    
    fn is_eql(&self, other: &Self) -> bool {
        if self.len != other.len { return false; }
        
        unsafe {
            let l = self.len;
            let mut i: usize = 0;
            
            if l / 8 > 1 {
                while i < l - (l % 8) {
                    let a = _mm_loadl_epi64(self.buff.as_ptr().add(i) as *const __m128i);
                    let b = _mm_loadl_epi64(other.buff.as_ptr().add(i) as *const __m128i);
                    
                    let mask = _mm_movemask_epi8(_mm_cmpeq_epi8(a, b));
                    
                    if (mask & 0xFF) != 0xFF { return false; }
                    i += 8;
                }
            }
            
            while i < l - 1 {
                if *self.buff.as_ptr().add(i) != *other.buff.as_ptr().add(i) { return false; }
                i += 1;
            }
            
            return true;
        }
    }
    
    pub fn start_with(&self, pattern: &str) -> bool {
        match self.index_of(*pattern.as_bytes().get(0).unwrap() as char) {
            Some(i) => {
                if i > 0 { return false; } else { i }
            },
            None => return false,
        };
        
        unsafe {
            for i in 1..pattern.len() {
                if *pattern.as_bytes().get(i).unwrap() != *self.buff.as_ptr().add(i) { return false; }
            }
        }
        
        return true;
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

impl AddAssign for DynamicString {
    fn add_assign(&mut self, mut rhs: Self) {
        self.append_str(rhs.as_str());
    }
}

impl PartialEq for DynamicString {
    fn eq(&self, other: &Self) -> bool {
        self.is_eql(other)
    }
}

impl Eq for DynamicString {}

impl Drop for DynamicString {
    fn drop(&mut self) {
        unsafe {
            if self.cap == 0 { return; }
            
            let layout = Layout::array::<u8>(self.cap).unwrap();
            dealloc(self.buff.as_ptr(), layout);
        }
    }
}