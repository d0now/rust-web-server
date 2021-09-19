use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

use std::sync::Arc;

use crate::config;
use crate::request;

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
pub async fn process(cfg: config::Config, std_listener: std::net::TcpListener) -> Result<()> {

    let cfg = Arc::new(cfg);

    let listener = TcpListener::from_std(std_listener)?;

    loop {
        let (mut client_stream, client_addr) = listener.accept().await?;
        let cfg = Arc::clone(&cfg);
        tokio::spawn(async move {
            if let Err(e) = request::process_requests(cfg, &mut client_stream, client_addr).await {
                eprintln!("Error occurred while processing request: {}", e);
            }
            client_stream.shutdown();
        });
    }
}