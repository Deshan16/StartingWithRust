mod dynstr;
mod sarr;
mod dynarr;

use std::arch::x86_64::{__m128i, _mm_loadl_epi64};

use dynstr::DynamicString;
use sarr::StaticArray;
use dynarr::DynamicArray;

fn main() {
    let mut x = DynamicString::new("Deshan");
    x.append_char(' ');
    x.append_str("Anjana");
    println!("{}", x);
    
    let s = DynamicString::new(" Jayasooriya!");
    x += s;
    println!("{}", x);
    x.pop();
    println!("{}", x);
    
    match x.lfind('6') {
        Some(i) => println!("{}", i),
        None => {},
    }
    
    let mut  y = StaticArray::<u8>::new(2);
    y.push(5).unwrap();
    y.push(5).unwrap();
    println!("{}", y);
    
    let mut z = DynamicArray::<f32>::reserve(2);
    z.push(25.6);
    z.push(25.6);
    z.push(25.6);
    println!("{}", z);
    z.pop();
    println!("{}", z);
    
    let a = DynamicArray::new([25.6, 34.8, 66.9]);
    println!("{}", a);
    println!("{}", a[1]);
    
    let str1 = DynamicString::new("Deshan Anjana");
    let str2 = DynamicString::new("Deshan Ankana");
    let str3 = DynamicString::new("Desn");
    let str4 = DynamicString::new("Deshan Anjana");
    
    println!("{}", str1 == str2);
    println!("{}", str1 == str3);
    println!("{}", str1 == str4);
}
