// #[derive(Default)] This will implement the predefined Default trait for predefined values
struct MyStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

// Here we are defining the values as need

impl Default for MyStruct{

    fn default() -> Self {
        MyStruct{
            field1: 30,
            field2: 20.4,
            field3: String::from("Hello"),
        }
    }
}

fn main() {
    // Create an instance with default values
    let my_struct_default: MyStruct = Default::default();

    // Access the fields
    println!("Field1: {}", my_struct_default.field1); // Output: Field1: 0
    println!("Field2: {}", my_struct_default.field2); // Output: Field2: 0
    println!("Field3: {}", my_struct_default.field3); // Output: Field3: (empty string)
}
