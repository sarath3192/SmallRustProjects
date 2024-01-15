

mod s;

fn print1(){
    println!("{}",s::MY_STATIC_STRING);
}

fn main() {
    print1();

    // The 'static lifetime is implicit for string literals
    let string_literal: &'static str = "I am a string literal.";
    println!("{}", string_literal);
}
