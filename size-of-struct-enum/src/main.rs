#![allow(warnings)]

use std::mem;
#[repr(align(8))] 
struct point {
    x_axis: f64,
    y_axis: f64,
}
#[repr(align(8))]
enum shape {
    cirlce(bool),
    rectagle(f64),
    squire(f64),
    polygon(f64),
}

fn main() {
    let size1 =  mem::size_of::<point>();
    let alignment_struct = mem::align_of::<point>();
    let size2 = mem::size_of::<shape>();
    let alignment_enum = mem::align_of::<shape>();
    println!("alignment of struct {}\nsize of struct {}\n\n",alignment_struct,size1);
    println!("alignment of enum {}\nsize of enum {}\n",alignment_enum,size2);
}