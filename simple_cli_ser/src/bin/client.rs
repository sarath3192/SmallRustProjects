use std::{io::{Read, Write}, net::TcpStream};
    
fn main() {
     let mut buff: [u8;6] = [0,1,0,1,0,1];
     println!("{:?}\n",buff);

     let client = TcpStream::connect("127.0.0.1:8080");

     match client {
        Ok(mut k) => {
            match k.write(&mut buff){
                Ok(n) => {
                    println!("{}",n);
                },
                Err(e) => {
                    println!("{}",e)
                },
            }
            match k.read(&mut buff) {
                Ok(n) =>{
                    println!("{}",n)
                },
                Err(e) => {
                    println!("{}",e)
                },
                
            }
            println!("{:?}",buff)
        },
        Err(e) => todo!(),
    }
}