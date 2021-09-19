use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use bytes::{BytesMut, BufMut};

use std::str;
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;

use crate::Result;
use crate::config;

struct Request<'a> {
    stream: &'a mut TcpStream,
    client_addr: SocketAddr,
    buffer: BytesMut,
}

struct RequestHeader {
    method: String,
    uri: String,
    version: String,
    keepalive: bool,
    headers: HashMap<String, String>,
}

enum RequestHeaderLine {
    Line((String, String)),
    Update,
    End,
}

impl<'a> Request<'a> {

    fn new(stream: &mut TcpStream, client_addr: SocketAddr) -> Request {
        Request {
            stream: stream,
            client_addr: client_addr,
            buffer: BytesMut::with_capacity(0x1000),
        }
    }

    async fn read_buffer(&mut self) -> Result<()> {
        if self.buffer.capacity() > 0 {
            self.stream.read_buf(&mut self.buffer).await?;
            Ok(())
        } else {
            Err("No more capacity.".into())
        }
    }

    fn update_buffer(&mut self, keep_at: usize) -> Option<u32> {
        self.buffer = self.buffer.split_off(keep_at);
        self.buffer.reserve(0x1000);
        Some(0)
    }

    async fn get_crlf_line(&mut self) -> Option<String> {

        let mut index = 0;
        let mut cr_found = false;
        let mut lf_found = false;

        let mut iter = &mut self.buffer[..].into_iter();
        for val in iter {
            index += 1;
            if cr_found {
                if *val == '\n' as u8 {
                    lf_found = true;
                    break;
                } else {
                    cr_found = false;
                }
            } else {
                if *val == '\r' as u8 {
                    cr_found = true;
                }
            }
        }

        if !(cr_found && lf_found) {
            return None;
        }

        let ret = match str::from_utf8(&self.buffer[..index-2]) {
            Ok(v) => Some(String::from(v)),
            Err(e) => None
        };

        self.update_buffer(index);
        return ret;
    }

    async fn get_request_line(&mut self) -> Result<(String, String, String)> {

        self.read_buffer().await?;

        let line = match self.get_crlf_line().await {
            Some(line) => line,
            None => { return Err("CRLF line not found.".into()); }
        };

        let v: Vec<&str> = line.split(' ').collect();
        if v.len() != 3 {
            return Err("Invalid header format.".into());
        }

        println!("{}", line);

        let method = v[0];
        let uri = v[1];
        let version = v[2];

        Ok((String::from(method), String::from(uri), String::from(version)))
    }
    
    async fn get_header_line(&mut self) -> Result<RequestHeaderLine> {
        
        let line = match self.get_crlf_line().await {
            Some(line) => line,
            None => {
                if self.buffer.capacity() > 0 {
                    return Ok(RequestHeaderLine::Update);
                } else {
                    return Err("CRLF line not found.".into());
                }
            }
        };

        if line.is_empty() {
            return Ok(RequestHeaderLine::End);
        }

        let (key, value) = match line.split_once(':') {
            Some(header) => header,
            None => {
                return Err("Invalid key/val.".into());
            }
        };

        Ok(RequestHeaderLine::Line((String::from(key.trim()), String::from(value.trim()))))
    }

    pub async fn process_header(&mut self) -> Result<RequestHeader> {

        let (method, uri, version) = match self.get_request_line().await {
            Ok(request_line) => request_line,
            Err(err) => { return Err(format!("Can't get request line: {}", err).into()); }
        };

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut keepalive = false;

        loop {
            let header_line = match self.get_header_line().await {
                Ok(header_line) => header_line,
                Err(err) => {
                    return Err(format!("Can't get header: {}", err).into());
                }
            };

            let (key, value) = match header_line {
                RequestHeaderLine::Line(header_line) => header_line,
                RequestHeaderLine::Update => { self.read_buffer().await; continue; },
                RequestHeaderLine::End => { break; },
            };

            println!("{}: {}", key, value);

            if key.to_lowercase() == "connection" && value.to_lowercase() == "keep-alive" {
                keepalive = true;
            }

            headers.insert(key, value);
        }

        Ok(RequestHeader {
            method: method,
            uri: uri,
            version: version,
            keepalive: keepalive,
            headers: headers,
        })
    }
}

pub async fn process_requests(
    cfg: Arc<config::Config>,
    stream: &mut TcpStream,
    addr: SocketAddr
) -> Result<()> {

    loop {
        // Start handle request.
        let mut req = Request::new(stream, addr);
        
        // Process header.
        let header = req.process_header().await?;

        stream.write_all(b"HTTP/1.1 200 OK\r\n").await;
        stream.write_all(b"Content-Length: 1\r\n").await;
        stream.write_all(b"\r\n").await;
        stream.write_all(b"O").await;
        if !header.keepalive {
            break;
        }
    }

    Ok(())
}