# Simple TLS connection

This code was created during class, basically creating a clone of this [repository](https://github.com/arthurazs/python-tls/tree/master) but on rust.


```bash
git clone https://github.com/TiagoPR/rust-tls.git
cd rust-tls/src
```

## Generate the certificate and private key

```bash
openssl req -new -x509 -days 365 -nodes -out cert.pem -keyout key.pem -subj "/C=BR/ST=Rio de Janeiro/L=Niteroi/O=UFF/OU=Midiacom/CN=example.org/"
```

Note that you can change some parameters:

- **C**, which is a 2 letter code for a country;
- **ST**, which is a state or province name;
- **L** *(optional)*, which is a city name;
- **O**, which is an organization name;
- **OU** *(optional)*, which is an organizational unit name;
- **CN**, which is the hostname:
  - **Warning** If you change the **CN** value, you have to change the hostname under [client.rs](src/client.rs) to reflect the new hostname.
- **emailAddress** *(optional)*, which is an email address.

## Running the TLS connection example

Open the terminal and enter the following commands:

```bash
cargo build
cargo run
```
