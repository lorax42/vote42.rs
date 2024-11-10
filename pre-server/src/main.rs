use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let addr = "127.0.0.1:4242"; // adress and port to listen on
    let listener = TcpListener::bind(addr).await?;
    println!("listening for SSH connection on {}", addr);

    loop {
        // accept incoming connections
        let (mut socket, _) = listener.accept().await?;
        println!("new connection established");

        // handle connection in a seperate task
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(0) => {
                    println!("connection closed");
                    return;
                }
                Ok(n) => {
                    // logic to handle SSH activity
                    println!("received {} bytes: {:?}", n, &buffer[..n]);
                }
                Err(e) => {
                    eprintln!("E: failed to read from socket: {}", e);
                    return;
                }
            }

            if let Err(e) = socket.write_all(b"hello from Rust SSH listener").await {
                eprintln!("failed to write to socket: {}", e);
            }
        });
    }
}
