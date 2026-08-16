use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
async fn main() -> std::io::Result<()>{
    let connect = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socet, addr) = connect.accept().await?;
        tokio::spawn(async move {

        });
    }
    Ok(())
}
async fn send_back() {
    
}
