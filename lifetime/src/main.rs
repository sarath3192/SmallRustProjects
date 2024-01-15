// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     let mut n = bytes.iter().enumerate();

//     // println!("{:?}\n",n);

//     for (i, &byte) in n {
//         if byte == b' ' {
//             return &s[..i];
//         }
//     }

//     &s
// }
// fn main() {
//     let my_string = String::from("Hello,R ust!");
//     let word = first_word(&my_string);
//     println!("The first word is: {}", word);
// }
struct Container<'a> {
    data: &'a str,
}

fn get_data<'a>() -> &'a str {
    let data = String::from("Hello, Rust!");
    &data
}

fn main() {
    let reference: &str;

    {
        let container = Container { data: get_data() };
        reference = container.data;
    }

    // ERROR: Use of possibly invalid reference outside its scope
    // println!("{}", reference);
}
