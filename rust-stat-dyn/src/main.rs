// trait MyTrait {  //this is a trait with one method in it
//     fn my_method(&self);
// }

// struct MyStruct; //It is a empty struct

// impl MyTrait for MyStruct { //Here we are implimenting the trait to the Mystruct
//     fn my_method(&self) {
//         println!("Calling my_method\n");
//     }
// }

// fn main() {
//     let my_instance = MyStruct;
//     my_instance.my_method();
//     let trait_object: &dyn MyTrait = &my_instance;
//     trait_object.my_method();
// }

fn main(){
let mut y = 10;
let reference_mut = &mut y; // Mutable reference to y
// let reference_mut1 = &mut y;

println!("Value of y before: {}", y);
*reference_mut = 20;
 // Modify the value through the mutable reference
// println!("Value of y after: {}", y);
}