use tokio::io::{self, AsyncReadExt, AsyncWriteExt}; 
use tokio::net::{TcpStream, TcpListener};

#[tokio::main] //改良过后的我的代码，主要问题出现在accept没有放入loop循环里
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    //let mut listener = TcpListener::bind("127.0.0.1:6142").await?; 
    /*let server = tokio::spawn(async move {
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
    });*/

    let client = tokio::spawn(async move {
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap(); 
        let (mut rd, mut wr) = socket.into_split(); 
        let mut line = String::new();
        loop {
            let input = std::io::stdin().read_line(&mut line).unwrap();
            wr.write_all(&line[0..line.len()]).await.unwrap(); 
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
        }
    });

    client.await.unwrap();
    Ok(())
}