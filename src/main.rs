use std::{io::BufReader, sync::Arc};
use rustls::pki_types::PrivateKeyDer;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
use tokio_rustls::TlsAcceptor;
#[tokio::main]
async fn main() -> std::io::Result<()>{
    //get private key:
    let key = load_key();
    //get certificates:
    let certs = load_certs();

    //create config (while empry): 
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth() //don't demand sertificate from client
        .with_single_cert(certs, key) // Server will be use THIS private and public key
        .unwrap();

    // A "tool" that wraps a TCP-conect to TLS
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
                    Ok(n) => {
                        println!("We get new user!");
                        n
                    },
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
//read sert.pem and return --> sertificates
fn load_certs() -> Vec<rustls::pki_types::CertificateDer<'static>>{
    //open file from path:
    let file = std::fs::File::open("cert.pem").unwrap();
    // "BufReader" better read file that just "File" so we use it:
    let mut reader = BufReader::new(file);
    // "rustls_pemfile" return iterator:
    rustls_pemfile::certs(&mut reader)
        .map(|cert| cert.unwrap())
        .collect()
}
//read key.pem and return --> private key
fn load_key() -> PrivateKeyDer<'static>{
    //open file from path
    let file = std::fs::File::open("key.pem").unwrap();
    // "BufReader" better read file that just "File" so we use it:
    let mut reader = BufReader::new(file);
    //read file and return the FIRST found private key:
    rustls_pemfile::private_key(&mut reader).unwrap().unwrap()
}
