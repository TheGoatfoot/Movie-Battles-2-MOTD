use std::net::UdpSocket;
use regex::bytes::Regex;

const PAYLOAD_HEADER: &[u8] = b"\xff\xff\xff\xffrcon ";

pub struct Rcon {
    socket: UdpSocket,
    host: String,
    payload_header: Vec<u8>,
    buf: [u8; 1024],
    regex_status: Regex
}

impl Rcon {
    pub fn new(password: String, host_address: String, host_port: u16, client_port: u16)->Rcon {
        let mut payload_header: Vec<u8> = Vec::from(PAYLOAD_HEADER);
        payload_header.append(&mut password.as_bytes().to_vec());
        Self {
            socket: UdpSocket::bind(format!("127.0.0.1:{}", client_port))
                .expect(&format!("cannot bind socket to port {}", client_port)),
            host: format!("{}:{}", host_address, host_port),
            payload_header: payload_header,
            buf: [0; 1024],
            regex_status: Regex::new(r"(?-u)\xff\xff\xff\xffprint\nBad rconpassword\.").unwrap()
        }
    }

    fn send(&mut self, command: String)->bool {
        let mut payload = Vec::new();
        payload.append(&mut self.payload_header.clone());
        payload.push(b' ');
        payload.append(&mut command.as_bytes().to_vec());
        let output = match self.socket.send_to(&payload, self.host.as_str()) {
            Ok(_)=>match self.socket.recv(&mut self.buf) {
                Ok(byte_count)=>{
                    if self.regex_status.is_match(&self.buf[..byte_count]) {
                        false
                    } else {
                        true
                    }
                },
                Err(_)=>false
            },
            Err(_)=>false
        };
        output
    }

    pub fn svsay(&mut self, message: String)->bool {
        self.send(format!("svsay {}", message))
    }
}
