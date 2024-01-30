//const TEST1: &[Num; 6] = &[Num{n: 2}, Num{n: 53}, Num{n: 51}, Num{n: 5}, Num{n: 50},Num{n: 52}];

/*************Iterator**************/
#[derive(Debug)]
struct root_test{
    num: Vec<i32>,
}

impl root_test{
    pub fn empty() -> Self {
        Self { num: Vec::new()}
    }
}

fn main(){

let mut t = root_test::empty();
t.num.push(1);
t.num.push(2);
println!("{:?}",t);
}
// #[derive(Debug,PartialEq)]                                           // use when data has to be shared safely between threads.
// struct Num{
//     n: u32,
// }
// const TEST1: &[Num; 6] = &[Num{n: 2}, Num{n: 53}, Num{n: 51}, Num{n: 5}, Num{n: 50},Num{n: 52}];

// fn main(){
    
//     let hello = "hello";
//     let hello2 = hello.clone();
//     assert_eq!(hello,hello2);
//     let i = TEST1.iter().clone();
//     assert_eq!(i, TEST1);

//     // println!("{:?}",i.next());

// }
// fn main() {
//     let some_ints = vec![1,2,3,4,5];
//     for i in &some_ints {
//         dbg!(i);
//     }
 
//     dbg!(some_ints);
//  }
 

// #[test]
// fn iter_demo() {
//     let v1 = vec![1, 2, 3];
//     let mut v1_iter = v1.iter();
//     // iter() returns an iterator of slices.
//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), Some(&3));
//     assert_eq!(v1_iter.next(), None);
    
// }

// #[test]
// fn into_iter_demo() {
//     let v1 = vec![1, 2, 3];
//     let mut v1_iter = v1.into_iter();

//     // into_iter() returns an iterator from a value.
//     assert_eq!(v1_iter.next(), Some(1));
//     assert_eq!(v1_iter.next(), Some(2));
//     assert_eq!(v1_iter.next(), Some(3));
//     assert_eq!(v1_iter.next(), None);
// }

// #[test]
// fn iter_mut_demo() {
//     let mut v1 = vec![1, 2, 3];
//     let mut v1_iter = v1.iter_mut();

//     // iter_mut() returns an iterator that allows modifying each value.
//     assert_eq!(v1_iter.next(), Some(&mut 1));
//     assert_eq!(v1_iter.next(), Some(&mut 2));
//     assert_eq!(v1_iter.next(), Some(&mut 3));
//     assert_eq!(v1_iter.next(), None);
// }




// fn main(){
//     let list = vec!["cat","mouse","dog","elephant","hourse"];

//     for i in list{
//         println!("{}",i);
//     }
// }

// #[derive(Debug)]
// struct general {
//     rol_no: i32,
//     name: String,
// }
// fn main() {
//     let mut t = vec![general{rol_no: 1, name: String::from("sarath")}];

//     t.push(general { rol_no: 2, name: String::from("kumar")});


//     for i in t{
//         println!("{:?}",i.rol_no);
//     }
// }

// fn main() {
// let num = (0..100).map(|x| x+1);
//     for i in num{
//         println!("{}",i);
//     }
// }

// fn main(){
//     let num = vec![1,3,4];

//     for i in &num{
//         println!("{:?}",i);
//     }
// }
// fn main() {
//     let num = vec![1,1,3,5];

//     for i in 0..num.len(){
//         println!("{}",num[i]);
//     }
// }

// fn main(){
// let mut range = 0..10;
// loop {
//     match range.next() {
//         Some(k)=> {
//         println!("{}",k)
//         }
//         None => {
//               break;
//         },
//      }
//   }
// }

// fn main(){

    // let x = [1, 2, 4];
    // let mut iterator = x.iter();
    // // let mut k: Vec<&i32> = iterator.collect();
    // let y = x;
    // println!("{:?}",y);
    // assert_eq!(iterator.next(), Some(&1));
    // assert_eq!(iterator.next(), Some(&2));
    // assert_eq!(iterator.next(), Some(&4));
    // assert_eq!(iterator.next(), None);
    // assert_eq!(iterator.next(), None);
    // println!("{:?}", x );

// }