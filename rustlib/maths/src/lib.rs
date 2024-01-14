pub mod sub_two_num;
pub mod add_two_num;

use add_two_num::{print_welcome, add};
use sub_two_num::{sub, print_leaving};

pub fn wel_add(a: usize, b: usize) -> usize{
   print_welcome();
   let c = add(a,b);
   println!("sum = {}\n",c);
   c
}

pub fn sub_bye(a: usize, b: usize) -> usize {
    
    let c = sub(a,b);
    println!("difference = {}\n", c);
    print_leaving();
    c
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_work() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}
