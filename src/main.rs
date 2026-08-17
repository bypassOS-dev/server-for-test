use std::{io::BufReader, sync::Arc};
use rustls::pki_types::PrivateKeyDer;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
use tokio_rustls::TlsAcceptor;
#[tokio::main]
async fn main() -> std::io::Result<()>{
    let key = load_key();
    let certs = load_certs();

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let connect = TcpListener::bind("127.0.0.1:8080").await?;
    println!("The server is working on 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = connect.accept().await?;
        let acceptor = acceptor.clone();
        tokio::spawn(async move {

            let mut tls_socket = match acceptor.accept(socket).await {
                Ok(s) => s,
                Err(err) => {
                    println!("TLS handshake error from {}: {}", addr, err);
                    return;
                }
            };

            let mut buffer = [0u8; 1024];

            loop {
                let read_bytes = match tls_socket.read(&mut buffer).await {
                       Ok(0) => {
                        println!("A client is completed a session!");
                        return;
                    }
                    Ok(n) => n,
                    Err(err) => {
                        println!("User read error: {err}");
                        return;
                    }
                };
                if let Err(e) = tls_socket.write_all(&buffer[..read_bytes]).await {
                    println!("Write error from {}: {}", addr, e);
                    return;
                }
            }
        });
    }
}
fn load_certs() -> Vec<rustls::pki_types::CertificateDer<'static>>{
    let file = std::fs::File::open("cert.pem").unwrap();
    let mut reader = BufReader::new(file);
    rustls_pemfile::certs(&mut reader)
        .map(|cert| cert.unwrap())
        .collect()
}
fn load_key() -> PrivateKeyDer<'static>{
    let file = std::fs::File::open("key.pem").unwrap();
    let mut reader = BufReader::new(file);
    rustls_pemfile::private_key(&mut reader).unwrap().unwrap()
}
