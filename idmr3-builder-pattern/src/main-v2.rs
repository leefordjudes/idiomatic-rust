#![allow(dead_code, unused_variables, non_camel_case_types)]

#[macro_use]
extern crate derive_builder;


#[derive(Debug, Clone)]
pub struct TLSCert {
    key: String,
    cert: String,
}

type ms = u32;

// struct only pub, its fields are private
#[derive(Debug, Builder, Default)]
pub struct Server {
    host: String, 
    port: u16,
    #[builder(default, setter(strip_option))]
    tls: Option<TLSCert>,
    #[builder(default)]
    hot_reload: bool,
    #[builder(default="2000")]
    timeout: ms,
}



fn main() {
    let host = "localhost".to_owned();
    let port = 8080;

    let cert = TLSCert {
        key: "...".to_owned(),
        cert: "...".to_owned(),
    };

    // Basic server
    let basic_server = ServerBuilder::default()
                            .host(host.clone())
                            .port(port)
                            .build()
                            .unwrap();
    println!("{:#?}", basic_server);
    // TLS server
    let tls_server = ServerBuilder::default()
                                .host(host.clone())
                                .port(port)
                                .tls(cert.clone())
                                .build()
                                .unwrap();
    println!("{:#?}", tls_server);
    // Advanced server
    let adv_server = ServerBuilder::default()
                                .host(host.clone())
                                .port(port)
                                .tls(cert.clone())
                                .hot_reload(true)
                                .timeout(5000)
                                .build()
                                .unwrap();
    println!("{:#?}", adv_server);                                
}
