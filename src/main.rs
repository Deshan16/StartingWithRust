mod dynstr;
mod sarr;
mod dynarr;

use dynstr::DynamicString;
use sarr::StaticArray;
use dynarr::DynamicArray;

fn main() {
    let mut x = DynamicString::new("Deshan");
    x.append_char(' ');
    x.append_str("Anjana");
    println!("{}", x);
    
    let mut  y = StaticArray::<u8>::new(2);
    y.push(5).unwrap();
    y.push(5).unwrap();
    
    println!("{}", y);
    
    let mut z = DynamicArray::<f32>::reserve(2);
    z.push(25.6);
    z.push(25.6);
    z.push(25.6);
    
    println!("{}", z);
}
