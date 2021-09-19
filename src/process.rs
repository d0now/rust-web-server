use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::config;
use crate::Result;

pub fn bind_server(cfg: &config::Config) -> Result<std::net::TcpListener> {

    let host = cfg.chk_get("Server", "Host");
    let port = cfg.chk_get("Server", "Port");

    match std::net::TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(listener) => Ok(listener),
        Err(err) => Err(err.into()),
    }
}

#[tokio::main]
pub async fn process(cfg: &config::Config, std_listener: std::net::TcpListener) -> Result<()> {

    let listener = TcpListener::from_std(std_listener)?;

    loop {
        let (mut client_stream, client_addr) = listener.accept().await?;
        println!("Connected: {}", client_addr.to_string());
        client_stream.shutdown().await?;
    }
}