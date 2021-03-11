use tokio::{net::{TcpStream}};
use tokio::task;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address: String = "localhost:35667".parse().unwrap();
    match TcpStream::connect(address).await {
        Ok(stream) => {
            match stream.try_write("message from client".as_bytes()) {
                Ok(n) => {
                    println!("The client send {} character to server", n);
                }
                Err(_) => {
                    println!("Something wrong happened when send data to stream!");
                }
            }
        }
        Err(_) => {
            println!("error happend when connecting to address!");
        }
    }
    Ok(())
}

/*#[tokio::main]
async fn main() -> io::Result<()> {
    let handler1 = task::spawn(async move {
        let address: String = "127.0.0.1:35667".parse().unwrap();
        match TcpStream::connect(address).await {
            Ok(stream) => {
                let stream  = stream;
                for i in 0..10 {
                    let mut data = "data from client".to_string();
                    data.push_str(&i.to_string());
                    match stream.try_write(data.as_bytes()) {
                        Ok(n) => {
                            println!("The client send {} character to server", n);
                        }
                        Err(_) => {
                            println!("Something wrong happened when send data to stream!");
                        }
                    }
                }
            }
            Err(_) => {
                println!("error happend when connecting to address!");
            }
        }
    });
    let handler2 = task::spawn(async move {
        let address: String = "127.0.0.1:35667".parse().unwrap();
        match TcpStream::connect(address).await {
            Ok(stream) => {
                let stream  = stream;
                loop {
                    let mut msg_rcv = Vec::<u8>::new();
                    match stream.try_read(&mut msg_rcv) {
                        Ok(0) => {}
                        Ok(n) => {
                            println!("client receive {} character from server", n);
                            println!("{}",String::from_utf8(msg_rcv).unwrap());
                        },
                        Err(_) => {
                            //println!("Something wrong happened when read data from stream!");
                        }
                    };
                }
            }
            Err(_) => {
                println!("error happend when connecting to address!");
            }
        }
    });
    let _ = tokio::join!(handler1, handler2);
    Ok(())
}*/
