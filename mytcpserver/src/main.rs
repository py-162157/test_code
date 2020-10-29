/*use tokio::io::{self, AsyncReadExt, AsyncWriteExt}; use tokio::net::{TcpStream, TcpListener};
use std::thread;
use std::time::Duration;
#[tokio::main] async fn main() -> io::Result<()> { let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap(); let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap(); let (mut rd, mut wr) = socket.into_split(); wr.write_all(b"message from 127.0.0.1!").await; let (mut socket, address) = listener.accept().await?;

tokio::spawn(async move {
    let mut buf = vec![0;1024];
    println!("the address connected is: {}", address);
        
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => {
                if socket.write_all(&buf[..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => {
                return;
            }
        }
    }
});

tokio::spawn(async move {
    let mut buffer = Vec::new();

    loop {
        match rd.read(&mut buffer).await {
            Ok(0) => return,
            Ok(_n) => println!("The received message's context is:{:?}", buffer),
            Err(_) => {
                println!("Error happened!");
                return;
            },
        }
    }
});
thread::sleep(Duration::from_secs(10));
Ok::<_, io::Error>(())
}*/

use tokio::io::{self, AsyncReadExt, AsyncWriteExt}; 
use tokio::net::{TcpStream, TcpListener};

#[tokio::main] //改良过后的我的代码，主要问题出现在accept没有放入loop循环里
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut listener = TcpListener::bind("127.0.0.1:6142").await?; 
    let server = tokio::spawn(async move {
        let mut buf = vec![0;1024];
        
        loop {
            let (mut socket, address) = listener.accept().await.unwrap();
            println!("the address connected is: {}", address);
            match socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => {
                    if socket.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    });

    /*tokio::spawn(async move {
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap(); 
        let (mut rd, mut wr) = socket.into_split(); 
        wr.write_all(b"message from 127.0.0.1!").await.unwrap(); 
        let mut buffer = String::new();

        loop {
            match rd.read_to_string(&mut buffer).await {
                Ok(0) => return,
                Ok(_n) => println!("The received message's context is:{}", buffer),
                Err(_) => {
                    println!("Error happened!");
                    return;
                },
            }
        }
    });*/

    server.await.unwrap();
    Ok(())
}

/*use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await?;
    let server = tokio::spawn(async move {
        println!("server: starting");
        let mut buf = vec![0; 1024];
        loop {
            let (mut listener_socket, address) = listener.accept().await.unwrap();
            println!("server: the address connected is: {}", address);
            match listener_socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => {
                    if listener_socket.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    });
    //网络解答者的代码
    tokio::task::spawn(async {
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
        let (mut rd, mut wr) = socket.into_split();
        println!("client: created.");
        wr.write_all(b"message from 127.0.0.1!").await.unwrap();
        let mut buffer = String::new();

        loop {
            match rd.read_to_string(&mut buffer).await {
                Ok(0) => return,
                Ok(_n) => println!("client: The received message's context is:{}", buffer),
                Err(_) => {
                    panic!("Error happened!");
                }
            }
        }
    });
    server.await.unwrap();
    Ok(())
}*/



/*use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};
//老师的代码
#[tokio::main]
async fn main() -> io::Result<()> {//implement a simple tcp server which can ehco my messages
    let t1 = tokio::spawn(async move {//processing code in my tcp server
        let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
        let mut buf = vec![0;1024];

        let (mut socket, address) = listener.accept().await.unwrap();
        println!("The accepted address is {}", &address);
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("server read nothing");
                    return;
                },
                Ok(n) => {
                    println!("server read {} bytes", n);
                    socket.write_all(&buf[..n]).await.unwrap();
                    return;
                }
                Err(_) => {
                    println!("server got an error");
                    return;
                }
            }
        }
    });

    let t2 = tokio::spawn(async move {//receive the returned message and print
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
        let (mut rd, mut wr) = socket.into_split();
        wr.write_all(b"message from PengYang!").await.unwrap();//send my message
        println!("am i here");
        let mut buffer = String::new();
        loop {
            match rd.read_to_string(&mut buffer).await {
                Ok(0) => {
                    println!("client read nothing");
                    return;
                },
                Ok(_n) => {
                    println!("client read {} bytes", _n);
                    println!("The received message's context is:{}", buffer);
                },
                Err(_) => {
                    println!("client got an error");
                    return;
                },
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    io::Result::Ok(())
}*/

