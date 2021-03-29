use glommio::LocalExecutorBuilder;

fn main() {
    let handle = LocalExecutorBuilder::new().spawn(|| async move {
        println!("hello");
    }).unwrap();
    
    handle.join().unwrap();
}