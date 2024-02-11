use rustls::crypto::{ring, CryptoProvider}; // This is a module in the rustls crate and it is used for crypto graphic operations
use std::io::{stdout, Read, Write};         // These Read and Write are the traits that are used by every type in the os
use std::net::TcpStream;                    // This module is used for the TCP connection establishment
use std::sync::Arc;                         // This Atomically reference counter, this uses automic to count the references give this 
 
                                            // use when data has to be shared safely between threads
fn main(){
    // let root_store = rustls::RootCertStore::from_iter(      // Here RootCertStore is initialized how
    //     webpki_roots::TLS_SERVER_ROOTS  let root_store = rustls::RootCertStore::from_iter(      // Here RootCertStore is initialized how
    //     webpki_roots::TLS_SERVER_ROOTS   // TLS_SERVER_ROOTS has constant data related to root certificates which is generated
    //         .iter()               // This will be used to convert the constant roots to iterable items
    //         .cloned(),                                     // This is used to create a copy of the roots so that we not modify the original constants
    // ); // TLS_SERVER_ROOTS has constant data related to root certificates which is generated
    //         .iter()               // This will be used to convert the constant roots to iterable items
    //         .cloned(),                                     // This is used to create a copy of the roots so that we not modify the original constants
    // );

let m =    CryptoProvider {
        cipher_suites: vec![ring::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256],
        kx_groups: vec![ring::kx_group::X25519],
        ..ring::default_provider()
    };
    println!("{:?}", m);
}

