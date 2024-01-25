use std::{io::Write, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // let port1 = TcpListener::local_addr(&listener);
    // println!("address+port {:?}", port1);

    // println!("Listen in on the bound addresss 127.0.0.1:8080\n");

    // let listen = listener.incoming();

    let buff:[u8; 6] = [1,3,3,6,8,7].into();

    println!("listening for the incomming stream\n");

    for stream in listener.incoming() {
        println!("{:?}",stream);
        match stream {
              Ok(mut k) => {

                match k.write(&buff){
                    Ok(n) => {
                    println!("{}",n)
                    }
                    Err(e) => {
                      println!("{}",e)
                    }
                }
                
              }
              Err(e) => {
                println!("connection lost\n")
              }

        }
    
        println!("Connection established!");
        // handle_client(stream);
        // let stream = stream.unwrap();
        // println!("Connection established!");
    }
}