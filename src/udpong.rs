use std::net::{SocketAddr, UdpSocket};

pub struct UdpPongServer {}

impl UdpPongServer {
    pub fn new() -> Self {
        UdpPongServer {}
    }

    pub fn run(self, listen_addr: impl Into<SocketAddr>) -> std::io::Result<()> {
        let socket = UdpSocket::bind(listen_addr.into())?;

        println!("Listen on {:?}", socket.local_addr().unwrap());

        loop {
            let mut buf = [0; 5];
            let (size, src) = socket.recv_from(&mut buf)?;
            if size == 5 && buf.starts_with(b"ping") {
                let packet_seq = buf[4];
                println!("Ping from {}: seq={}", src, packet_seq);
                buf[0] = b'p';
                buf[1] = b'o';
                buf[2] = b'n';
                buf[3] = b'g';
                buf[4] = packet_seq;
                socket.send_to(&buf, &src)?;
            }
        }
    }
}
