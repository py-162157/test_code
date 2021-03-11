/*use tokio::net::{TcpStream};
use tokio::task;
use tokio::time::sleep;
use std::io;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() -> io::Result<()> {
    let address: String = "0.0.0.0:35667".parse().unwrap();
    let stream = TcpStream::connect(address).await?;
    let mut rng = thread_rng();
    let mut rand_numbers = Vec::<i32>::new();
    for _ in 0..100 {
        rand_numbers.push(rng.gen_range(1..5));
    }
    //let (rx, tx) = stream.split();
    

    let handler = task::spawn(async move {
        loop {
            let process = async {
                let mut count = 0;
                let mut msg_rcv = Vec::<u8>::new();
                match stream.try_read(&mut msg_rcv) {
                    Ok(0) => {}
                    Ok(n) => {
                        println!("The server receive {} character", n);
                        let mut msg_send:String = String::from_utf8(msg_rcv).unwrap();
                        msg_send.push_str("appendix from server");
                        sleep(std::time::Duration::from_secs(rand_numbers[count] as u64)).await;
                        count += 1;
                        match stream.try_write(msg_send.as_bytes()) {
                            Ok(_) => {}
                            Err(_) => {
                                println!("Something wrong happened when write data to stream!");
                            }
                        }
                    },
                    Err(_) => {
                        //println!("Something wrong happened when read data from stream!");
                    }
                };
            };
            
            process.await;
        }
    });
    handler.await?;
    Ok(())
}*/

use std::time::Duration;

use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, time::sleep}; 
use tokio::net::{TcpStream, TcpListener};

#[tokio::main] async fn main() -> io::Result<()> { 
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap(); 
    let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap(); 
    let (mut rd, mut wr) = socket.into_split(); 

    let handler1 = tokio::spawn(async move {
        loop {
            let mut buf = vec![0;4096];
            let (mut socket, address) = listener.accept().await.unwrap();
            println!("the address connected is: {}", address);
            match socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(_) => {
                    print!("The server successfully receive a message from client!\n");
                    let mut str_data = String::from_utf8(buf).unwrap();
                    let _ = sleep(Duration::from_secs(3));
                    str_data.push_str(", reply");
                    //let _ = tokio::time::sleep(std::time::Duration::from_secs(3));
                    if socket.write_all(&str_data.as_bytes()).await.is_err() {
                        print!("An error occurred when send message back to client!");
                        return;
                    }
                }
                Err(_) => {
                    print!("An error occurred when read message from client!");
                    return;
                }
            }
        }
    });

    let handler2 = tokio::spawn(async move {
        let mut buffer = String::new();
        for i in 0..10 {
            let mut data = "data from client".to_string();
            data.push_str(&i.to_string());
            match wr.write_all(data.as_bytes()).await {
                Ok(_) => {
                    println!("The client successfully send characters to server");
                }
                Err(_) => {
                    println!("Something wrong happened when send data to stream!");
                }
            }
        }

        loop {
            match rd.read_to_string(&mut buffer).await {
                Ok(0) => return,
                Ok(_) => println!("{}", buffer),
                Err(_) => {
                    println!("Error happened!");
                    return;
                },
            }
        }
    });

    handler2.await?;
    handler1.await?;
    Ok::<_, io::Error>(())
}

