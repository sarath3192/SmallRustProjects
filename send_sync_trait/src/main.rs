// // Define a custom type
// struct MyData {
//     value: i32,
//     e: String,
// }

// // Implement the `Send` trait for `MyData`
// // unsafe impl Send for MyData {}

// fn main() {
//     // Create an instance of `MyData`
//     let data = MyData { value: 42 , e: String::from("hello")};

//     // Spawn a new thread and move `data` into it
//     let handle = std::thread::spawn(move || {
//         // Access `data` in the new thread
//         println!("Value in the new thread: {}", data.e);
//     });

//     // Wait for the spawned thread to finish
//     handle.join().unwrap();

//     // Attempting to use `data` here would result in a compilation error,
//     // because `data` has been moved to the spawned thread.
//     // Uncommenting the line below would result in a compilation error:
//     // println!("Value in the main thread: {}", data.value);
// }
struct MyNonSendType {
    data: String,
}

fn main() {
    // Create an instance of a non-Send type
    let non_send_instance = MyNonSendType {
        data: String::from("Hello, World!"),
    };

    // Attempt to move the non-Send type to a new thread
    std::thread::spawn(move || {
        // This line attempts to access non_send_instance.data in a different thread,
        // and it would result in a compilation error because MyNonSendType is not Send.
        println!("Data in the new thread: {}", non_send_instance.data);
    }).join().unwrap();
}
