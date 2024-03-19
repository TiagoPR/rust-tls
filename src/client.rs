use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn client() {
    let hostname = "example.org";
    let ip = "127.0.0.1";
    let port = 8443;

    let mut connect = SslConnector::builder(SslMethod::tls_client()).unwrap();
    connect.set_ca_file("cert.pem").unwrap();
    let connect = connect.build();

    let stream = TcpStream::connect(format!("{ip}:{port}")).unwrap();
    let mut tls = connect.connect(hostname, stream).unwrap();

    tls.write_all(b"Hello, world").unwrap();

    // let data = tls.get_ref().read(&mut [0; 1024]).unwrap();
    // println!("Server says: {}", data.to_string().as_str());

    // stream.write_all(b"Hello, world").unwrap();

    let mut buffer = [0; 1024];
    let bytes_read = tls.read(&mut buffer).unwrap();
    println!(
        "Server says: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
}
