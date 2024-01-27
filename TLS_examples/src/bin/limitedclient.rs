use rustls::crypto::{ring, CryptoProvider}; // This is a module in the rustls crate and it is used for crypto graphic operations
use std::io::{stdout, Read, Write};         // These Read and Write are the traits that are used by every type in the os
use std::net::TcpStream;                    // This module is used for the TCP connection establishment
use std::sync::Arc;                         // This Atomically reference counter, this uses automic to count the references give this 
 
 #[derive(Debug)]                                           // use when data has to be shared safely between threads.
struct Num{
    n: u32,
}
const TEST1: &[Num; 6] = &[Num{n: 2}, Num{n: 53}, Num{n: 51}, Num{n: 5}, Num{n: 50},Num{n: 52}]; // constants will not change at run time
// fn main(){
    
   

//     // let root_store = rustls::RootCertStore::from_iter(
//     //     webpki_roots::TLS_SERVER_ROOTS
//     //         .iter()
//     //         .cloned(),
//     // );
// }

