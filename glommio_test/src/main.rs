use glommio::{Async, LocalExecutor};
use glommio::timer::Timer;
use futures_lite::{future::FutureExt, io};

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    let local_ex = LocalExecutor::make_default();
    local_ex.run(async {
        let addr = "127.0.0.1110::80".to_socket_addrs()?.next().unwrap();
        let stream = Async::<TcpStream>::connect(addr).or(async {
            Timer::new(Duration::from_secs(10)).await;
            Err(io::ErrorKind::TimedOut.into())
        })
        .await?;
        std::io::Result::Ok(())
    });
}



