#![allow(dead_code, unused_variables, non_camel_case_types)]

#[derive(Debug, Clone)]
struct TLSCert {
    key: String,
    cert: String,
}

type ms = u32;

// struct only pub, its fields are private
#[derive(Debug)]
pub struct Server {
    host: String, 
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: bool,
    timeout: ms,
}

impl Server {
    fn new(host: String, port: u16) -> Self {
        Self {host, port, tls: None, hot_reload: false, timeout: 2000}
    }
    
    // rust not support function overloading: multiple fn with same name, diff args.
    fn new_tls(host: String, port: u16, tls: TLSCert) -> Self {
        Self {host, port, tls: Some(tls), hot_reload: false, timeout: 2000}
    }

    fn new_advanced(
        host: String, 
        port: u16, 
        tls: Option<TLSCert>,
        hot_reload: bool,
        timeout: ms
    ) -> Self {
        Self {host, port, tls, hot_reload, timeout}
    }
}

fn main() {
    let host = "localhost".to_owned();
    let port = 8080;

    let cert = TLSCert {
        key: "...".to_owned(),
        cert: "...".to_owned(),
    };

    // Basic server
    let basic_server = Server::new(host.clone(), port);

    // TLS server
    let tls_server = Server::new_tls(host.clone(), port, cert.clone());

    // Advanced server
    let adv_server = Server::new_advanced(
        host.clone(),
        port,
        Some(cert.clone()),
        true,
        5000
    );



}
