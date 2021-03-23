use glommio::LocalExecutor;
use glommio::timer::Timer;
use futures_lite::{future::FutureExt};
use glommio::net::TcpStream;

use std::time::Duration;
use std::io;

fn main()-> Result<(), std::io::Error> {
    let local_ex = LocalExecutor::default();
    let _  = local_ex.run(async {
        let timeout = async {
            Timer::new(Duration::from_secs(10)).await;
            Err(io::Error::new(io::ErrorKind::TimedOut, "").into())
        };
        let stream = TcpStream::connect("127.0.0.1000::80").or(timeout).await?;
        std::io::Result::Ok(())
    });
    Ok(())
}



