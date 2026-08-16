use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() -> std::io::Result<()>{
    let connect = TcpListener::bind("127.0.0.1:8080").await?;
    println!("The server is working on 127.0.0.1:8080");
    loop {
        let (mut socet, addr) = connect.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            //let mut some = String::new();
            //std::io::stdin()
            //    .read_line(&mut some)
            //    .expect("FATAL Error read of string");

            let read_bytes = match socet.read(&mut buffer).await {
                Ok(0) => {
                    println!("A client is completed a session!");
                }
                Ok(n) => n,
                Err(err) => {
                    println!("User read error: {err}")
                }
            };

        });
    }
    Ok(())
}

