use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

pub fn server() {
    let ip = "127.0.0.1";
    let port = 8443;

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    acceptor.set_certificate_chain_file("cert.pem").unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = acceptor.build();

    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut stream = acceptor.accept(stream).unwrap();
                    //let data = stream.get_ref().read(&mut [0; 1024]).unwrap();
                    //println!("Client says: {}", data.to_string().as_str());
                    let mut buffer = [0; 1024];
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    println!(
                        "Client says: {}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                    stream.write_all(b"Ur welcome").unwrap();
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}
