use std::net::{ToSocketAddrs, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

pub struct UdpPingClient {
    timeout: Duration,
}

impl UdpPingClient {
    pub fn new() -> Self {
        UdpPingClient {
            timeout: Duration::new(3, 0),
        }
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn run<A: ToSocketAddrs>(self, remote_addr: A) -> std::io::Result<()> {
        let remote_addr = remote_addr
            .to_socket_addrs()?
            .next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unable to resolve address",
            ))?;

        println!("UDPING {:?}", remote_addr);

        let socket = UdpSocket::bind(("0.0.0.0", 0))?;

        socket.set_read_timeout(Some(self.timeout))?;

        socket.connect(remote_addr)?;

        let mut next_packet_seq: u8 = 1;
        loop {
            let start = Instant::now();
            let mut buf = [0; 5];
            buf[0] = b'p';
            buf[1] = b'i';
            buf[2] = b'n';
            buf[3] = b'g';
            buf[4] = next_packet_seq;
            socket.send(&buf)?;
            match socket.recv(&mut buf) {
                Ok(size) => {
                    if size == 5 && buf.starts_with(b"pong") {
                        println!(
                            "Pong from {}: seq={} time={:.3} ms",
                            socket.peer_addr().unwrap(),
                            buf[4],
                            start.elapsed().as_secs_f32() * 1000.0
                        )
                    }
                }
                Err(err) => {
                    println!("Ping {:?} for seq {}", err.kind(), next_packet_seq);
                }
            }

            let interval = Duration::new(1, 0);
            let elapsed = start.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }

            next_packet_seq = next_packet_seq.wrapping_add(1);
        }
    }
}
